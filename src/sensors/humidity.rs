//! Module providing humidity sensor functionality.

use super::{LocalisedSensorMetadata, SensorMetadata, SensorTemplate, Sensors};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct HumiditySensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct HumiditySensorTemplate {
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl TryInto<HumiditySensor> for HumiditySensorTemplate {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<HumiditySensor, Self::Error> {
        Ok(HumiditySensor {
            metadata: self.metadata.try_into()?,
            unit: self.unit,
            ..HumiditySensor::default()
        })
    }
}

impl SensorTemplate for HumiditySensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: HumiditySensor = self.clone().try_into()?;
        sensor.value = value_str.parse::<f64>()?;
        sensors.humidity.push(sensor);
        Ok(())
    }
}
