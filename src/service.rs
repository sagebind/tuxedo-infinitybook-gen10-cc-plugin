use crate::{
    SERVICE_ID, VERSION,
    device_service::v1::{
        CustomFunctionOneRequest, CustomFunctionOneResponse, EnableManualFanControlRequest,
        EnableManualFanControlResponse, FixedDutyRequest, FixedDutyResponse, HealthRequest,
        HealthResponse, InitializeDeviceRequest, InitializeDeviceResponse, LcdRequest, LcdResponse,
        LightingRequest, LightingResponse, ListDevicesRequest, ListDevicesResponse,
        ResetChannelRequest, ResetChannelResponse, ShutdownRequest, ShutdownResponse,
        SpeedProfileRequest, SpeedProfileResponse, StatusRequest, StatusResponse,
        device_service_server::DeviceService, health_response,
    },
    models::{
        self,
        v1::{
            ChannelInfo, Device, DeviceInfo, SpeedOptions, channel_info::Options, status::FanSpeed,
        },
    },
    tuxedo_io::{Fan, TuxedoIo},
};
use std::{collections::HashMap, io, sync::Arc};
use sysinfo::Product;
use tokio::task::spawn_blocking;
use tonic::{Request, Response, Status};

const DEVICE_ID: &str = "tuxedo";
const FAN_1_CHANNEL_ID: &str = "fan1";
const FAN_2_CHANNEL_ID: &str = "fan2";

pub struct TuxedoService {
    device: Device,
    tuxedo_io: Arc<TuxedoIo>,
}

impl TuxedoService {
    pub fn new() -> io::Result<Self> {
        let tuxedo_io = Arc::new(TuxedoIo::open()?);
        let min_duty = tuxedo_io.get_fan_min_speed()?.into();
        let max_duty = tuxedo_io.get_fan_max_speed()?.into();

        let mut channels = HashMap::new();

        channels.insert(
            FAN_1_CHANNEL_ID.into(),
            ChannelInfo {
                label: Some("Fan 1".into()),
                options: Some(Options::SpeedOptions(SpeedOptions {
                    min_duty,
                    max_duty,
                    fixed_enabled: true,
                    ..Default::default()
                })),
            },
        );

        channels.insert(
            FAN_2_CHANNEL_ID.into(),
            ChannelInfo {
                label: Some("Fan 2".into()),
                options: Some(Options::SpeedOptions(SpeedOptions {
                    min_duty,
                    max_duty,
                    fixed_enabled: true,
                    ..Default::default()
                })),
            },
        );

        Ok(Self {
            device: Device {
                id: DEVICE_ID.into(),
                name: Product::name().unwrap_or_else(|| "TUXEDO InfinityBook Gen10".into()),
                uid_info: None,
                info: Some(DeviceInfo {
                    channels,
                    ..Default::default()
                }),
            },
            tuxedo_io,
        })
    }

    async fn invoke_blocking<T: Send + 'static>(
        &self,
        f: impl Send + FnOnce(&TuxedoIo) -> T + 'static,
    ) -> Result<T, Status> {
        let tuxedo_io = self.tuxedo_io.clone();

        spawn_blocking(move || f(tuxedo_io.as_ref()))
            .await
            .map_err(|e| Status::from_error(Box::new(e)))
    }
}

#[tonic::async_trait]
impl DeviceService for TuxedoService {
    /// Used to confirm service connection and retrieve service health information.
    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let reply = HealthResponse {
            name: SERVICE_ID.to_string(),
            version: VERSION.to_string(),
            status: health_response::Status::Ok.into(),
            // information purposes only
            uptime_seconds: 1,
        };
        Ok(Response::new(reply))
    }

    /// This is the first message sent to the device service after establishing a connection
    /// and is used to detect the service's devices and capabilities.
    /// The device models should be filled out for each device and all of their
    /// available channels. This information is used to populate the CoolerControl device
    /// list and available features in the UI.
    async fn list_devices(
        &self,
        _request: Request<ListDevicesRequest>,
    ) -> Result<Response<ListDevicesResponse>, Status> {
        Ok(Response::new(ListDevicesResponse {
            devices: vec![self.device.clone()],
        }))
    }

    /// This is called and used by some devices to initialize hardware, before starting to send
    /// commands to it. It is also be called after resuming from sleep, as many firmwares are rest.
    async fn initialize_device(
        &self,
        _request: Request<InitializeDeviceRequest>,
    ) -> Result<Response<InitializeDeviceResponse>, Status> {
        Ok(Response::new(InitializeDeviceResponse {}))
    }

    async fn shutdown(
        &self,
        _request: Request<ShutdownRequest>,
    ) -> Result<Response<ShutdownResponse>, Status> {
        // TODO: Device shutdown logic
        // Note: The CoolerControl daemon will initiate a service termination after this point.
        Ok(Response::new(ShutdownResponse {}))
    }

    /// This is called to retrieve the status per device and their respective channels
    /// and is called at a regular intervals (default 1 second).
    ///
    /// Device _channels_ usually can not be done concurrently, but that depends on the hardware and drivers.
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let tuxedo_io = self.tuxedo_io.clone();

        spawn_blocking(move || {
            Ok(Response::new(StatusResponse {
                status: vec![
                    models::v1::Status {
                        id: FAN_1_CHANNEL_ID.into(),
                        metric: Some(models::v1::status::Metric::Speed(FanSpeed {
                            duty: Some(tuxedo_io.get_fan_speed(Fan::Fan1)? as f64),
                            rpm: None,
                        })),
                    },
                    models::v1::Status {
                        id: FAN_2_CHANNEL_ID.into(),
                        metric: Some(models::v1::status::Metric::Speed(FanSpeed {
                            duty: Some(tuxedo_io.get_fan_speed(Fan::Fan2)? as f64),
                            rpm: None,
                        })),
                    },
                ],
            }))
        })
        .await
        .map_err(|e| Status::from_error(Box::new(e)))
        .flatten()
    }

    /// Reset the device channel to it's default state if applicable. (Auto)
    async fn reset_channel(
        &self,
        _request: Request<ResetChannelRequest>,
    ) -> Result<Response<ResetChannelResponse>, Status> {
        let tuxedo_io = self.tuxedo_io.clone();

        spawn_blocking(move || {
            tuxedo_io.set_fans_auto()?;

            Ok(Response::new(ResetChannelResponse {}))
        })
        .await
        .map_err(|e| Status::from_error(Box::new(e)))
        .flatten()
    }

    async fn enable_manual_fan_control(
        &self,
        _request: Request<EnableManualFanControlRequest>,
    ) -> Result<Response<EnableManualFanControlResponse>, Status> {
        // Nothing to do, will automatically activate manual control when setting
        // a speed.
        Ok(Response::new(EnableManualFanControlResponse {}))
    }

    async fn fixed_duty(
        &self,
        request: Request<FixedDutyRequest>,
    ) -> Result<Response<FixedDutyResponse>, Status> {
        self.invoke_blocking(move |tuxedo_io| {
            let fan = if request.get_ref().channel_id == FAN_1_CHANNEL_ID {
                Fan::Fan1
            } else if request.get_ref().channel_id == FAN_2_CHANNEL_ID {
                Fan::Fan2
            } else {
                return Err(Status::invalid_argument("Unknown channel ID"));
            };

            tuxedo_io.set_fan_speed(fan, request.get_ref().duty as u8)?;

            Ok(Response::new(FixedDutyResponse {}))
        })
        .await
        .flatten()
    }

    async fn speed_profile(
        &self,
        _request: Request<SpeedProfileRequest>,
    ) -> Result<Response<SpeedProfileResponse>, Status> {
        // TODO: Apply a speed profile to the device channel
        Err(Status::unimplemented("No Firmware Profiles"))
    }

    async fn lighting(
        &self,
        _request: Request<LightingRequest>,
    ) -> Result<Response<LightingResponse>, Status> {
        // TODO: Apply a lighting mode to the device channel
        Err(Status::unimplemented("No Lighting Channels"))
    }

    async fn lcd(&self, _request: Request<LcdRequest>) -> Result<Response<LcdResponse>, Status> {
        // TODO: Apply a LCD mode
        Err(Status::unimplemented("No LCD Channels"))
    }

    /// This is a placeholder for any custom functions that the device service might expose.
    async fn custom_function_one(
        &self,
        _request: Request<CustomFunctionOneRequest>,
    ) -> Result<Response<CustomFunctionOneResponse>, Status> {
        Err(Status::unimplemented("No Custom Function"))
    }
}
