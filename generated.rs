// Generated file DO NOT EDIT
use crate::connection;
use crate::connection::Connection;
use crate::decoder;
use crate::encoder;
use crate::error;

use super::schema;

pub struct SpaceCenter<'a> {
    conn: &'a Connection,
}
impl<'a> SpaceCenter<'a> {
    pub fn new(conn: &'a Connection) -> SpaceCenter<'a> {
        SpaceCenter {
            conn,
        }
    }

    // service methods
    pub async fn clear_target(&'a self) -> Result<(), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "ClearTarget", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn launchable_vessels(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "LaunchableVessels", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn launch_vessel(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "LaunchVessel", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn launch_vessel_from_vab(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "LaunchVesselFromVAB", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn launch_vessel_from_sph(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "LaunchVesselFromSPH", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn save(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "Save", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn load(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "Load", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn quicksave(&'a self) -> Result<(), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "Quicksave", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn quickload(&'a self) -> Result<(), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "Quickload", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn can_rails_warp_at(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "CanRailsWarpAt", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn warp_to(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "WarpTo", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn transform_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "TransformPosition", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn transform_direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "TransformDirection", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn transform_rotation(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "TransformRotation", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn transform_velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "TransformVelocity", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn raycast_distance(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "RaycastDistance", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn raycast_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "RaycastPart", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    // getters
    pub async fn get_game_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_GameMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_science(&'a self) -> Result<f32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Science", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_funds(&'a self) -> Result<f64, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Funds", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_reputation(&'a self) -> Result<f32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Reputation", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_active_vessel(&'a self) -> Result<Vessel<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_ActiveVessel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    pub async fn get_vessels(&'a self) -> Result<(/*list*/), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Vessels", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_bodies(&'a self) -> Result<(/*dict*/), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Bodies", arguments).await?;
        let return_value = decoder::decode_dictionary(result)?;
        Ok(return_value)
    }

    pub async fn get_target_body(&'a self) -> Result<CelestialBody<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_TargetBody", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CelestialBody{id: return_value, conn: &self.conn})
    }

    pub async fn get_target_vessel(&'a self) -> Result<Vessel<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_TargetVessel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    pub async fn get_target_docking_port(&'a self) -> Result<DockingPort<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_TargetDockingPort", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(DockingPort{id: return_value, conn: &self.conn})
    }

    pub async fn get_waypoint_manager(&'a self) -> Result<WaypointManager<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_WaypointManager", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(WaypointManager{id: return_value, conn: &self.conn})
    }

    pub async fn get_contract_manager(&'a self) -> Result<ContractManager<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_ContractManager", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ContractManager{id: return_value, conn: &self.conn})
    }

    pub async fn get_camera(&'a self) -> Result<Camera<'a>, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Camera", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Camera{id: return_value, conn: &self.conn})
    }

    pub async fn get_ui_visible(&'a self) -> Result<bool, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_UIVisible", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_navball(&'a self) -> Result<bool, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_Navball", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_ut(&'a self) -> Result<f64, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_UT", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_g(&'a self) -> Result<f64, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_G", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_warp_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_WarpMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_warp_rate(&'a self) -> Result<f32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_WarpRate", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_warp_factor(&'a self) -> Result<f32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_WarpFactor", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_rails_warp_factor(&'a self) -> Result<i32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_RailsWarpFactor", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_physics_warp_factor(&'a self) -> Result<i32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_PhysicsWarpFactor", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_maximum_rails_warp_factor(&'a self) -> Result<i32, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_MaximumRailsWarpFactor", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_far_available(&'a self) -> Result<bool, error::Error> {
        let arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "get_FARAvailable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    
    // setters
    pub async fn set_active_vessel(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_target_body(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_target_vessel(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_target_docking_port(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_ui_visible(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_navball(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_rails_warp_factor(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_physics_warp_factor(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("SpaceCenter", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

}

// Classes
#[derive(Debug)]
pub struct Antenna<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Antenna<'a> {
    // methods
    pub async fn transmit(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn cancel(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_deployable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_Deployable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Antenna_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_can_transmit(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_CanTransmit", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_allow_partial(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_AllowPartial", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_allow_partial(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Antenna_set_AllowPartial", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_power(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_Power", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_combinable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_Combinable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_combinable_exponent(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_CombinableExponent", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_packet_interval(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_PacketInterval", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_packet_size(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_PacketSize", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_packet_resource_cost(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Antenna_get_PacketResourceCost", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct AutoPilot<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> AutoPilot<'a> {
    // methods
    pub async fn engage(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn disengage(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn wait(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn target_pitch_and_heading(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_error(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_Error", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_pitch_error(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_PitchError", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_heading_error(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_HeadingError", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_roll_error(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_RollError", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn set_reference_frame(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_target_pitch(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_TargetPitch", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_target_pitch(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_TargetPitch", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_target_heading(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_TargetHeading", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_target_heading(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_TargetHeading", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_target_roll(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_TargetRoll", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_target_roll(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_TargetRoll", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_target_direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_TargetDirection", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_target_direction(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_TargetDirection", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_sas(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_SAS", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_sas(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_SAS", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_sas_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_SASMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn set_sas_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_enumeration()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_SASMode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_roll_threshold(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_RollThreshold", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_roll_threshold(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_RollThreshold", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_stopping_time(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_StoppingTime", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_stopping_time(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_StoppingTime", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_deceleration_time(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_DecelerationTime", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_deceleration_time(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_DecelerationTime", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_attenuation_angle(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_AttenuationAngle", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_attenuation_angle(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_AttenuationAngle", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_auto_tune(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_AutoTune", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_auto_tune(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_AutoTune", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_time_to_peak(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_TimeToPeak", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_time_to_peak(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_TimeToPeak", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_overshoot(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_Overshoot", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_overshoot(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_Overshoot", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_pitch_pid_gains(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_PitchPIDGains", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_pitch_pid_gains(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_PitchPIDGains", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_roll_pid_gains(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_RollPIDGains", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_roll_pid_gains(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_RollPIDGains", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_yaw_pid_gains(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_get_YawPIDGains", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_yaw_pid_gains(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "AutoPilot_set_YawPIDGains", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Camera<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Camera<'a> {
    // methods
    // getters
    pub async fn get_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_Mode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn set_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_enumeration()?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_Mode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_pitch(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_Pitch", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_pitch(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_Pitch", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_heading(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_Heading", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_heading(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_Heading", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_distance(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_Distance", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_distance(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_Distance", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_min_pitch(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_MinPitch", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_pitch(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_MaxPitch", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_min_distance(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_MinDistance", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_distance(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_MaxDistance", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_default_distance(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_DefaultDistance", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_focussed_body(&'a self) -> Result<CelestialBody<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_FocussedBody", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CelestialBody{id: return_value, conn: &self.conn})
    }

    pub async fn set_focussed_body(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_FocussedBody", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_focussed_vessel(&'a self) -> Result<Vessel<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_FocussedVessel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    pub async fn set_focussed_vessel(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_FocussedVessel", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_focussed_node(&'a self) -> Result<Node<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Camera_get_FocussedNode", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Node{id: return_value, conn: &self.conn})
    }

    pub async fn set_focussed_node(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "Camera_set_FocussedNode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct CargoBay<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> CargoBay<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CargoBay_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CargoBay_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_open(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CargoBay_get_Open", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_open(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "CargoBay_set_Open", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct CelestialBody<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> CelestialBody<'a> {
    // methods
    pub async fn surface_height(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn bedrock_height(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn msl_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn surface_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn bedrock_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn position_at_altitude(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn latitude_at_position(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn longitude_at_position(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn altitude_at_position(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn atmospheric_density_at_position(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn temperature_at(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn density_at(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn pressure_at(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn biome_at(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn rotation(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn angular_velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_satellites(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_Satellites", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_mass(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_Mass", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_gravitational_parameter(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_GravitationalParameter", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_surface_gravity(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_SurfaceGravity", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_rotational_period(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_RotationalPeriod", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_rotational_speed(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_RotationalSpeed", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_rotation_angle(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_RotationAngle", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_initial_rotation(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_InitialRotation", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_equatorial_radius(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_EquatorialRadius", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_sphere_of_influence(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_SphereOfInfluence", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_orbit(&'a self) -> Result<Orbit<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_Orbit", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Orbit{id: return_value, conn: &self.conn})
    }

    pub async fn get_has_atmosphere(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_HasAtmosphere", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_atmosphere_depth(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_AtmosphereDepth", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_has_atmospheric_oxygen(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_HasAtmosphericOxygen", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_biomes(&'a self) -> Result<(/*set*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_Biomes", arguments).await?;
        let return_value = decoder::decode_set(result)?;
        Ok(return_value)
    }

    pub async fn get_flying_high_altitude_threshold(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_FlyingHighAltitudeThreshold", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_space_high_altitude_threshold(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_SpaceHighAltitudeThreshold", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_non_rotating_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_NonRotatingReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_orbital_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CelestialBody_get_OrbitalReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct CommLink<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> CommLink<'a> {
    // methods
    // getters
    pub async fn get_type(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommLink_get_Type", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_signal_strength(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommLink_get_SignalStrength", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_start(&'a self) -> Result<CommNode<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommLink_get_Start", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CommNode{id: return_value, conn: &self.conn})
    }

    pub async fn get_end(&'a self) -> Result<CommNode<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommLink_get_End", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CommNode{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct CommNode<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> CommNode<'a> {
    // methods
    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommNode_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_is_home(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommNode_get_IsHome", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_is_control_point(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommNode_get_IsControlPoint", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_is_vessel(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommNode_get_IsVessel", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_vessel(&'a self) -> Result<Vessel<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CommNode_get_Vessel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Comms<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Comms<'a> {
    // methods
    // getters
    pub async fn get_can_communicate(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Comms_get_CanCommunicate", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_can_transmit_science(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Comms_get_CanTransmitScience", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_signal_strength(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Comms_get_SignalStrength", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_signal_delay(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Comms_get_SignalDelay", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_power(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Comms_get_Power", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_control_path(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Comms_get_ControlPath", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Contract<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Contract<'a> {
    // methods
    pub async fn cancel(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn accept(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn decline(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_type(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Type", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_title(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Title", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_description(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Description", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_notes(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Notes", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_synopsis(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Synopsis", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_keywords(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Keywords", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_failed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Failed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_seen(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Seen", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_read(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Read", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_can_be_canceled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_CanBeCanceled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_can_be_declined(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_CanBeDeclined", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_can_be_failed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_CanBeFailed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_funds_advance(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_FundsAdvance", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_funds_completion(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_FundsCompletion", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_funds_failure(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_FundsFailure", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_reputation_completion(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_ReputationCompletion", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_reputation_failure(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_ReputationFailure", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_science_completion(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_ScienceCompletion", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_parameters(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Contract_get_Parameters", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ContractManager<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ContractManager<'a> {
    // methods
    // getters
    pub async fn get_types(&'a self) -> Result<(/*set*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractManager_get_Types", arguments).await?;
        let return_value = decoder::decode_set(result)?;
        Ok(return_value)
    }

    pub async fn get_all_contracts(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractManager_get_AllContracts", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_active_contracts(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractManager_get_ActiveContracts", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_offered_contracts(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractManager_get_OfferedContracts", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_completed_contracts(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractManager_get_CompletedContracts", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_failed_contracts(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractManager_get_FailedContracts", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ContractParameter<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ContractParameter<'a> {
    // methods
    // getters
    pub async fn get_title(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_Title", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_notes(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_Notes", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_children(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_Children", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_completed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_Completed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_failed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_Failed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_optional(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_Optional", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_funds_completion(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_FundsCompletion", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_funds_failure(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_FundsFailure", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_reputation_completion(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_ReputationCompletion", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_reputation_failure(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_ReputationFailure", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_science_completion(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ContractParameter_get_ScienceCompletion", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Control<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Control<'a> {
    // methods
    pub async fn activate_next_stage(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_action_group(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_action_group(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn toggle_action_group(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn add_node(&'a self) -> Result<Node<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Node{id: return_value, conn: &self.conn})
    }

    pub async fn remove_nodes(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_source(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Source", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_sas(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_SAS", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_sas(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_SAS", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_sas_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_SASMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn set_sas_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_enumeration()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_SASMode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_speed_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_SpeedMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn set_speed_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_enumeration()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_SpeedMode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_rcs(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_RCS", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_rcs(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_RCS", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_reaction_wheels(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_ReactionWheels", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_reaction_wheels(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_ReactionWheels", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_gear(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Gear", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_gear(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Gear", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_legs(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Legs", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_legs(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Legs", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_wheels(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Wheels", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_wheels(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Wheels", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_lights(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Lights", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_lights(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Lights", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_brakes(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Brakes", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_brakes(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Brakes", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_antennas(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Antennas", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_antennas(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Antennas", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_cargo_bays(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_CargoBays", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_cargo_bays(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_CargoBays", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_intakes(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Intakes", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_intakes(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Intakes", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_parachutes(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Parachutes", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_parachutes(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Parachutes", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_radiators(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Radiators", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_radiators(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Radiators", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_resource_harvesters(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_ResourceHarvesters", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_resource_harvesters(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_ResourceHarvesters", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_resource_harvesters_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_ResourceHarvestersActive", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_resource_harvesters_active(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_ResourceHarvestersActive", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_solar_panels(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_SolarPanels", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_solar_panels(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_SolarPanels", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_abort(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Abort", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_abort(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Abort", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_throttle(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Throttle", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_throttle(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Throttle", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_input_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_InputMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn set_input_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_enumeration()?,
        });
        let result = self.conn.execute_procedure("", "Control_set_InputMode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_pitch(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Pitch", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_pitch(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Pitch", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_yaw(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Yaw", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_yaw(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Yaw", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_roll(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Roll", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_roll(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Roll", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_forward(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Forward", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_forward(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Forward", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_up(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Up", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_up(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Up", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_right(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Right", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_right(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_Right", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_wheel_throttle(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_WheelThrottle", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_wheel_throttle(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_WheelThrottle", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_wheel_steering(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_WheelSteering", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_wheel_steering(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Control_set_WheelSteering", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_current_stage(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_CurrentStage", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_nodes(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Control_get_Nodes", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ControlSurface<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ControlSurface<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_pitch_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_PitchEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_pitch_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_set_PitchEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_yaw_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_YawEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_yaw_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_set_YawEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_roll_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_RollEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_roll_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_set_RollEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_authority_limiter(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_AuthorityLimiter", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_authority_limiter(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_set_AuthorityLimiter", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_inverted(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_Inverted", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_inverted(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_set_Inverted", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_surface_area(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_SurfaceArea", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_available_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ControlSurface_get_AvailableTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct CrewMember<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> CrewMember<'a> {
    // methods
    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_name(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_string()?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_set_Name", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_type(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Type", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_on_mission(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_OnMission", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_courage(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Courage", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_courage(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_set_Courage", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_stupidity(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Stupidity", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_stupidity(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_set_Stupidity", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_experience(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Experience", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_experience(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_set_Experience", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_badass(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Badass", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_badass(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_set_Badass", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_veteran(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_get_Veteran", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_veteran(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "CrewMember_set_Veteran", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Decoupler<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Decoupler<'a> {
    // methods
    pub async fn decouple(&'a self) -> Result<Vessel<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Decoupler_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_decoupled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Decoupler_get_Decoupled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_staged(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Decoupler_get_Staged", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Decoupler_get_Impulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct DockingPort<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> DockingPort<'a> {
    // methods
    pub async fn undock(&'a self) -> Result<Vessel<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    pub async fn position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn rotation(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_docked_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_DockedPart", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_reengage_distance(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_ReengageDistance", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_has_shield(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_HasShield", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_shielded(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_Shielded", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_shielded(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_set_Shielded", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "DockingPort_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Engine<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Engine<'a> {
    // methods
    pub async fn toggle_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_active(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Engine_set_Active", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Thrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_available_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_AvailableThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_MaxThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_vacuum_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_MaxVacuumThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thrust_limit(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_ThrustLimit", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_thrust_limit(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Engine_set_ThrustLimit", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_thrusters(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Thrusters", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_SpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_vacuum_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_VacuumSpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_kerbin_sea_level_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_KerbinSeaLevelSpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_propellant_names(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_PropellantNames", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_propellants(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Propellants", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_propellant_ratios(&'a self) -> Result<(/*dict*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_PropellantRatios", arguments).await?;
        let return_value = decoder::decode_dictionary(result)?;
        Ok(return_value)
    }

    pub async fn get_has_fuel(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_HasFuel", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_throttle(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Throttle", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_throttle_locked(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_ThrottleLocked", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_can_restart(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_CanRestart", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_can_shutdown(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_CanShutdown", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_has_modes(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_HasModes", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_mode(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Mode", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_mode(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_string()?,
        });
        let result = self.conn.execute_procedure("", "Engine_set_Mode", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_modes(&'a self) -> Result<(/*dict*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Modes", arguments).await?;
        let return_value = decoder::decode_dictionary(result)?;
        Ok(return_value)
    }

    pub async fn get_auto_mode_switch(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_AutoModeSwitch", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_auto_mode_switch(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Engine_set_AutoModeSwitch", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_gimballed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_Gimballed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_gimbal_range(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_GimbalRange", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_gimbal_locked(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_GimbalLocked", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_gimbal_locked(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Engine_set_GimbalLocked", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_gimbal_limit(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_GimbalLimit", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_gimbal_limit(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Engine_set_GimbalLimit", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_available_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Engine_get_AvailableTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Experiment<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Experiment<'a> {
    // methods
    pub async fn run(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn transmit(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn dump(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn reset(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_inoperable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Inoperable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_rerunnable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Rerunnable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_has_data(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_HasData", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_data(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Data", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_available(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Available", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_biome(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_Biome", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_science_subject(&'a self) -> Result<ScienceSubject<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Experiment_get_ScienceSubject", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ScienceSubject{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Fairing<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Fairing<'a> {
    // methods
    pub async fn jettison(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Fairing_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_jettisoned(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Fairing_get_Jettisoned", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Flight<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Flight<'a> {
    // methods
    pub async fn simulate_aerodynamic_force_at(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_g_force(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_GForce", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_mean_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_MeanAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_surface_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_SurfaceAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_bedrock_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_BedrockAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_elevation(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Elevation", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_latitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Latitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_longitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Longitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Velocity", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_speed(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Speed", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_horizontal_speed(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_HorizontalSpeed", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_vertical_speed(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_VerticalSpeed", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_center_of_mass(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_CenterOfMass", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_rotation(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Rotation", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Direction", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_pitch(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Pitch", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_heading(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Heading", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_roll(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Roll", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_prograde(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Prograde", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_retrograde(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Retrograde", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_normal(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Normal", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_anti_normal(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_AntiNormal", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_radial(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Radial", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_anti_radial(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_AntiRadial", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_atmosphere_density(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_AtmosphereDensity", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_dynamic_pressure(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_DynamicPressure", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_static_pressure_at_msl(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_StaticPressureAtMSL", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_static_pressure(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_StaticPressure", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_aerodynamic_force(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_AerodynamicForce", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_lift(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Lift", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_drag(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Drag", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_speed_of_sound(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_SpeedOfSound", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_mach(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_Mach", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_reynolds_number(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_ReynoldsNumber", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_true_air_speed(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_TrueAirSpeed", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_equivalent_air_speed(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_EquivalentAirSpeed", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_terminal_velocity(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_TerminalVelocity", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_angle_of_attack(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_AngleOfAttack", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_sideslip_angle(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_SideslipAngle", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_total_air_temperature(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_TotalAirTemperature", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_static_air_temperature(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_StaticAirTemperature", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_stall_fraction(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_StallFraction", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_drag_coefficient(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_DragCoefficient", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_lift_coefficient(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_LiftCoefficient", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_ballistic_coefficient(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_BallisticCoefficient", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thrust_specific_fuel_consumption(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Flight_get_ThrustSpecificFuelConsumption", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Force<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Force<'a> {
    // methods
    pub async fn remove(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Force_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_force_vector(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Force_get_ForceVector", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_force_vector(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "Force_set_ForceVector", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Force_get_Position", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_position(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "Force_set_Position", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Force_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn set_reference_frame(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "Force_set_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Intake<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Intake<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Intake_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_open(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Intake_get_Open", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_open(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Intake_set_Open", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_speed(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Intake_get_Speed", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_flow(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Intake_get_Flow", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_area(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Intake_get_Area", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct LaunchClamp<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> LaunchClamp<'a> {
    // methods
    pub async fn release(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "LaunchClamp_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Leg<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Leg<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Leg_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Leg_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_deployable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Leg_get_Deployable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Leg_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Leg_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_is_grounded(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Leg_get_IsGrounded", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Light<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Light<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Light_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Light_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_active(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Light_set_Active", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_color(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Light_get_Color", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_color(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "Light_set_Color", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_power_usage(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Light_get_PowerUsage", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Module<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Module<'a> {
    // methods
    pub async fn has_field(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_field(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_field_int(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_field_float(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn set_field_string(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn reset_field(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn has_event(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn trigger_event(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn has_action(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_action(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Module_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Module_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_fields(&'a self) -> Result<(/*dict*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Module_get_Fields", arguments).await?;
        let return_value = decoder::decode_dictionary(result)?;
        Ok(return_value)
    }

    pub async fn get_events(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Module_get_Events", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_actions(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Module_get_Actions", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Node<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Node<'a> {
    // methods
    pub async fn burn_vector(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn remaining_burn_vector(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn remove(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_prograde(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_Prograde", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_prograde(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Node_set_Prograde", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_normal(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_Normal", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_normal(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Node_set_Normal", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_radial(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_Radial", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_radial(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Node_set_Radial", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_delta_v(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_DeltaV", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_delta_v(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Node_set_DeltaV", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_remaining_delta_v(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_RemainingDeltaV", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_ut(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_UT", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_ut(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Node_set_UT", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_time_to(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_TimeTo", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_orbit(&'a self) -> Result<Orbit<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_Orbit", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Orbit{id: return_value, conn: &self.conn})
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_orbital_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Node_get_OrbitalReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Orbit<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Orbit<'a> {
    // methods
    pub async fn reference_plane_normal(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn reference_plane_direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn mean_anomaly_at_ut(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn radius_at_true_anomaly(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn true_anomaly_at_radius(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn true_anomaly_at_ut(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn ut_at_true_anomaly(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn eccentric_anomaly_at_ut(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn orbital_speed_at(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn radius_at(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn position_at(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn time_of_closest_approach(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn distance_at_closest_approach(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn list_closest_approaches(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn true_anomaly_at_an(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn true_anomaly_at_dn(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn relative_inclination(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_body(&'a self) -> Result<CelestialBody<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Body", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CelestialBody{id: return_value, conn: &self.conn})
    }

    pub async fn get_apoapsis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Apoapsis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_periapsis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Periapsis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_apoapsis_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_ApoapsisAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_periapsis_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_PeriapsisAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_semi_major_axis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_SemiMajorAxis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_semi_minor_axis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_SemiMinorAxis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_radius(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Radius", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_speed(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Speed", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_period(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Period", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_time_to_apoapsis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_TimeToApoapsis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_time_to_periapsis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_TimeToPeriapsis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_eccentricity(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Eccentricity", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_inclination(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Inclination", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_longitude_of_ascending_node(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_LongitudeOfAscendingNode", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_argument_of_periapsis(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_ArgumentOfPeriapsis", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_mean_anomaly_at_epoch(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_MeanAnomalyAtEpoch", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_epoch(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_Epoch", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_mean_anomaly(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_MeanAnomaly", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_eccentric_anomaly(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_EccentricAnomaly", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_true_anomaly(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_TrueAnomaly", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_next_orbit(&'a self) -> Result<Orbit<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_NextOrbit", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Orbit{id: return_value, conn: &self.conn})
    }

    pub async fn get_time_to_soi_change(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_TimeToSOIChange", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_orbital_speed(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Orbit_get_OrbitalSpeed", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Parachute<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Parachute<'a> {
    // methods
    pub async fn deploy(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn arm(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_armed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_get_Armed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_deploy_altitude(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_get_DeployAltitude", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_deploy_altitude(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_set_DeployAltitude", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_deploy_min_pressure(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_get_DeployMinPressure", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_deploy_min_pressure(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Parachute_set_DeployMinPressure", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Part<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Part<'a> {
    // methods
    pub async fn position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn center_of_mass(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn bounding_box(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn rotation(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn add_force(&'a self) -> Result<Force<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Force{id: return_value, conn: &self.conn})
    }

    pub async fn instantaneous_force(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_title(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Title", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_tag(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Tag", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_tag(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_string()?,
        });
        let result = self.conn.execute_procedure("", "Part_set_Tag", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_highlighted(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Highlighted", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_highlighted(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Part_set_Highlighted", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_highlight_color(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_HighlightColor", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn set_highlight_color(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_tuple()?,
        });
        let result = self.conn.execute_procedure("", "Part_set_HighlightColor", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_cost(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Cost", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_vessel(&'a self) -> Result<Vessel<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Vessel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Vessel{id: return_value, conn: &self.conn})
    }

    pub async fn get_parent(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Parent", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_children(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Children", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_axially_attached(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_AxiallyAttached", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_radially_attached(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_RadiallyAttached", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_stage(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Stage", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_decouple_stage(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_DecoupleStage", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_massless(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Massless", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_mass(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Mass", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_dry_mass(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_DryMass", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_shielded(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Shielded", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_dynamic_pressure(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_DynamicPressure", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_impact_tolerance(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ImpactTolerance", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_temperature(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Temperature", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_skin_temperature(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_SkinTemperature", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_max_temperature(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_MaxTemperature", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_max_skin_temperature(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_MaxSkinTemperature", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_mass(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalMass", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_skin_mass(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalSkinMass", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_resource_mass(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalResourceMass", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_internal_flux(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalInternalFlux", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_conduction_flux(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalConductionFlux", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_convection_flux(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalConvectionFlux", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_radiation_flux(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalRadiationFlux", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_skin_to_internal_flux(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ThermalSkinToInternalFlux", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_resources(&'a self) -> Result<Resources<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Resources", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Resources{id: return_value, conn: &self.conn})
    }

    pub async fn get_crossfeed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Crossfeed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_is_fuel_line(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_IsFuelLine", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_fuel_lines_from(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_FuelLinesFrom", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_fuel_lines_to(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_FuelLinesTo", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_modules(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Modules", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_antenna(&'a self) -> Result<Antenna<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Antenna", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Antenna{id: return_value, conn: &self.conn})
    }

    pub async fn get_cargo_bay(&'a self) -> Result<CargoBay<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_CargoBay", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CargoBay{id: return_value, conn: &self.conn})
    }

    pub async fn get_control_surface(&'a self) -> Result<ControlSurface<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ControlSurface", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ControlSurface{id: return_value, conn: &self.conn})
    }

    pub async fn get_decoupler(&'a self) -> Result<Decoupler<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Decoupler", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Decoupler{id: return_value, conn: &self.conn})
    }

    pub async fn get_docking_port(&'a self) -> Result<DockingPort<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_DockingPort", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(DockingPort{id: return_value, conn: &self.conn})
    }

    pub async fn get_engine(&'a self) -> Result<Engine<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Engine", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Engine{id: return_value, conn: &self.conn})
    }

    pub async fn get_experiment(&'a self) -> Result<Experiment<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Experiment", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Experiment{id: return_value, conn: &self.conn})
    }

    pub async fn get_fairing(&'a self) -> Result<Fairing<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Fairing", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Fairing{id: return_value, conn: &self.conn})
    }

    pub async fn get_intake(&'a self) -> Result<Intake<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Intake", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Intake{id: return_value, conn: &self.conn})
    }

    pub async fn get_leg(&'a self) -> Result<Leg<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Leg", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Leg{id: return_value, conn: &self.conn})
    }

    pub async fn get_launch_clamp(&'a self) -> Result<LaunchClamp<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_LaunchClamp", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(LaunchClamp{id: return_value, conn: &self.conn})
    }

    pub async fn get_light(&'a self) -> Result<Light<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Light", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Light{id: return_value, conn: &self.conn})
    }

    pub async fn get_parachute(&'a self) -> Result<Parachute<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Parachute", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Parachute{id: return_value, conn: &self.conn})
    }

    pub async fn get_radiator(&'a self) -> Result<Radiator<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Radiator", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Radiator{id: return_value, conn: &self.conn})
    }

    pub async fn get_rcs(&'a self) -> Result<RCS<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_RCS", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(RCS{id: return_value, conn: &self.conn})
    }

    pub async fn get_reaction_wheel(&'a self) -> Result<ReactionWheel<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ReactionWheel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReactionWheel{id: return_value, conn: &self.conn})
    }

    pub async fn get_resource_converter(&'a self) -> Result<ResourceConverter<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ResourceConverter", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ResourceConverter{id: return_value, conn: &self.conn})
    }

    pub async fn get_resource_harvester(&'a self) -> Result<ResourceHarvester<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ResourceHarvester", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ResourceHarvester{id: return_value, conn: &self.conn})
    }

    pub async fn get_sensor(&'a self) -> Result<Sensor<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Sensor", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Sensor{id: return_value, conn: &self.conn})
    }

    pub async fn get_solar_panel(&'a self) -> Result<SolarPanel<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_SolarPanel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(SolarPanel{id: return_value, conn: &self.conn})
    }

    pub async fn get_wheel(&'a self) -> Result<Wheel<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_Wheel", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Wheel{id: return_value, conn: &self.conn})
    }

    pub async fn get_moment_of_inertia(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_MomentOfInertia", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_inertia_tensor(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_InertiaTensor", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_center_of_mass_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Part_get_CenterOfMassReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Parts<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Parts<'a> {
    // methods
    pub async fn with_name(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn with_title(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn with_tag(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn with_module(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn in_stage(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn in_decouple_stage(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn modules_with_name(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_all(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_All", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_root(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Root", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_controlling(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Controlling", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn set_controlling(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "Parts_set_Controlling", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_antennas(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Antennas", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_control_surfaces(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_ControlSurfaces", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_cargo_bays(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_CargoBays", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_decouplers(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Decouplers", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_docking_ports(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_DockingPorts", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_engines(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Engines", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_experiments(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Experiments", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_fairings(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Fairings", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_intakes(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Intakes", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_legs(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Legs", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_launch_clamps(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_LaunchClamps", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_lights(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Lights", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_parachutes(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Parachutes", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_radiators(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Radiators", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_rcs(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_RCS", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_reaction_wheels(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_ReactionWheels", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_resource_converters(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_ResourceConverters", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_resource_harvesters(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_ResourceHarvesters", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_sensors(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Sensors", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_solar_panels(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_SolarPanels", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_wheels(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Parts_get_Wheels", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Propellant<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Propellant<'a> {
    // methods
    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_current_amount(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_CurrentAmount", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_current_requirement(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_CurrentRequirement", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_total_resource_available(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_TotalResourceAvailable", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_total_resource_capacity(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_TotalResourceCapacity", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_ignore_for_isp(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_IgnoreForIsp", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_ignore_for_thrust_curve(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_IgnoreForThrustCurve", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_draw_stack_gauge(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_DrawStackGauge", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_is_deprived(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_IsDeprived", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_ratio(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Propellant_get_Ratio", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct RCS<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> RCS<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_Enabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_Enabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_pitch_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_PitchEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_pitch_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_PitchEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_yaw_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_YawEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_yaw_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_YawEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_roll_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_RollEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_roll_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_RollEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_forward_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_ForwardEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_forward_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_ForwardEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_up_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_UpEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_up_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_UpEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_right_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_RightEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_right_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "RCS_set_RightEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_available_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_AvailableTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_max_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_MaxThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_vacuum_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_MaxVacuumThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thrusters(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_Thrusters", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_SpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_vacuum_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_VacuumSpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_kerbin_sea_level_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_KerbinSeaLevelSpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_propellants(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_Propellants", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_propellant_ratios(&'a self) -> Result<(/*dict*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_PropellantRatios", arguments).await?;
        let return_value = decoder::decode_dictionary(result)?;
        Ok(return_value)
    }

    pub async fn get_has_fuel(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "RCS_get_HasFuel", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Radiator<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Radiator<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Radiator_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_deployable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Radiator_get_Deployable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Radiator_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Radiator_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Radiator_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ReactionWheel<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ReactionWheel<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ReactionWheel_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ReactionWheel_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_active(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ReactionWheel_set_Active", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_broken(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ReactionWheel_get_Broken", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_available_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ReactionWheel_get_AvailableTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_max_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ReactionWheel_get_MaxTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ReferenceFrame<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ReferenceFrame<'a> {
    // methods
    pub async fn create_relative(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn create_hybrid(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    // getters
    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Resource<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Resource<'a> {
    // methods
    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_max(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_Max", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_amount(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_Amount", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_density(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_Density", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_flow_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_FlowMode", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resource_get_Enabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Resource_set_Enabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ResourceConverter<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ResourceConverter<'a> {
    // methods
    pub async fn active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn start(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn stop(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn status_info(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn inputs(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn outputs(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceConverter_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_count(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceConverter_get_Count", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_efficiency(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceConverter_get_ThermalEfficiency", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_core_temperature(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceConverter_get_CoreTemperature", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_optimum_core_temperature(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceConverter_get_OptimumCoreTemperature", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ResourceHarvester<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ResourceHarvester<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_active(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_set_Active", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_extraction_rate(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_ExtractionRate", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thermal_efficiency(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_ThermalEfficiency", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_core_temperature(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_CoreTemperature", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_optimum_core_temperature(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceHarvester_get_OptimumCoreTemperature", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ResourceTransfer<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ResourceTransfer<'a> {
    // methods
    pub async fn start(&'a self) -> Result<ResourceTransfer<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ResourceTransfer{id: return_value, conn: &self.conn})
    }

    // getters
    pub async fn get_complete(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceTransfer_get_Complete", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_amount(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ResourceTransfer_get_Amount", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Resources<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Resources<'a> {
    // methods
    pub async fn with_resource(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn has_resource(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn max(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn amount(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn density(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn flow_mode(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_all(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resources_get_All", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_names(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resources_get_Names", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Resources_get_Enabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Resources_set_Enabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ScienceData<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ScienceData<'a> {
    // methods
    // getters
    pub async fn get_data_amount(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceData_get_DataAmount", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_science_value(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceData_get_ScienceValue", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_transmit_value(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceData_get_TransmitValue", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct ScienceSubject<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> ScienceSubject<'a> {
    // methods
    // getters
    pub async fn get_science(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_Science", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_science_cap(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_ScienceCap", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_is_complete(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_IsComplete", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_data_scale(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_DataScale", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_scientific_value(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_ScientificValue", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_subject_value(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_SubjectValue", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_title(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "ScienceSubject_get_Title", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Sensor<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Sensor<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Sensor_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_active(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Sensor_get_Active", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_active(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Sensor_set_Active", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_value(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Sensor_get_Value", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct SolarPanel<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> SolarPanel<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_deployable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_get_Deployable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_energy_flow(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_get_EnergyFlow", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_sun_exposure(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "SolarPanel_get_SunExposure", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Thruster<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Thruster<'a> {
    // methods
    pub async fn thrust_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn thrust_direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn initial_thrust_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn initial_thrust_direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn gimbal_position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Thruster_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_thrust_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Thruster_get_ThrustReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_gimballed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Thruster_get_Gimballed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_gimbal_angle(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Thruster_get_GimbalAngle", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Vessel<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Vessel<'a> {
    // methods
    pub async fn recover(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn flight(&'a self) -> Result<Flight<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Flight{id: return_value, conn: &self.conn})
    }

    pub async fn resources_in_decouple_stage(&'a self) -> Result<Resources<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Resources{id: return_value, conn: &self.conn})
    }

    pub async fn position(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn bounding_box(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn rotation(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn direction(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn angular_velocity(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    // getters
    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_name(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_string()?,
        });
        let result = self.conn.execute_procedure("", "Vessel_set_Name", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_type(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Type", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn set_type(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_enumeration()?,
        });
        let result = self.conn.execute_procedure("", "Vessel_set_Type", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_situation(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Situation", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_recoverable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Recoverable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_met(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_MET", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn get_biome(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Biome", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn get_orbit(&'a self) -> Result<Orbit<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Orbit", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Orbit{id: return_value, conn: &self.conn})
    }

    pub async fn get_control(&'a self) -> Result<Control<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Control", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Control{id: return_value, conn: &self.conn})
    }

    pub async fn get_comms(&'a self) -> Result<Comms<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Comms", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Comms{id: return_value, conn: &self.conn})
    }

    pub async fn get_auto_pilot(&'a self) -> Result<AutoPilot<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AutoPilot", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(AutoPilot{id: return_value, conn: &self.conn})
    }

    pub async fn get_crew_capacity(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_CrewCapacity", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_crew_count(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_CrewCount", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_crew(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Crew", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_resources(&'a self) -> Result<Resources<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Resources", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Resources{id: return_value, conn: &self.conn})
    }

    pub async fn get_parts(&'a self) -> Result<Parts<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Parts", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Parts{id: return_value, conn: &self.conn})
    }

    pub async fn get_mass(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Mass", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_dry_mass(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_DryMass", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_Thrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_available_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_MaxThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_max_vacuum_thrust(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_MaxVacuumThrust", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_SpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_vacuum_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_VacuumSpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_kerbin_sea_level_specific_impulse(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_KerbinSeaLevelSpecificImpulse", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_moment_of_inertia(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_MomentOfInertia", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_inertia_tensor(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_InertiaTensor", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_available_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_available_reaction_wheel_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableReactionWheelTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_available_rcs_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableRCSTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_available_engine_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableEngineTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_available_control_surface_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableControlSurfaceTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_available_other_torque(&'a self) -> Result<(/*tuple*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_AvailableOtherTorque", arguments).await?;
        let return_value = decoder::decode_tuple(result)?;
        Ok(return_value)
    }

    pub async fn get_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_ReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_orbital_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_OrbitalReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_surface_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_SurfaceReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    pub async fn get_surface_velocity_reference_frame(&'a self) -> Result<ReferenceFrame<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Vessel_get_SurfaceVelocityReferenceFrame", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(ReferenceFrame{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Waypoint<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Waypoint<'a> {
    // methods
    pub async fn remove(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    // getters
    pub async fn get_body(&'a self) -> Result<CelestialBody<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Body", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(CelestialBody{id: return_value, conn: &self.conn})
    }

    pub async fn set_body(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_u64(0)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_Body", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_name(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Name", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_name(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_string()?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_Name", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_color(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Color", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn set_color(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_sint32()?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_Color", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_icon(&'a self) -> Result<String, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Icon", arguments).await?;
        let return_value = decoder::decode_string(result)?;
        Ok(return_value)
    }

    pub async fn set_icon(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_string()?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_Icon", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_latitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Latitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_latitude(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_Latitude", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_longitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Longitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_longitude(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_Longitude", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_mean_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_MeanAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_mean_altitude(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_MeanAltitude", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_surface_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_SurfaceAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_surface_altitude(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_SurfaceAltitude", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_bedrock_altitude(&'a self) -> Result<f64, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_BedrockAltitude", arguments).await?;
        let return_value = decoder::decode_double(result)?;
        Ok(return_value)
    }

    pub async fn set_bedrock_altitude(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_double(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_set_BedrockAltitude", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_near_surface(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_NearSurface", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_grounded(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Grounded", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_index(&'a self) -> Result<i32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Index", arguments).await?;
        let return_value = decoder::decode_sint32(result)?;
        Ok(return_value)
    }

    pub async fn get_clustered(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Clustered", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_has_contract(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_HasContract", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_contract(&'a self) -> Result<Contract<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Waypoint_get_Contract", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Contract{id: return_value, conn: &self.conn})
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct WaypointManager<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> WaypointManager<'a> {
    // methods
    pub async fn add_waypoint(&'a self) -> Result<Waypoint<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Waypoint{id: return_value, conn: &self.conn})
    }

    pub async fn add_waypoint_at_altitude(&'a self) -> Result<Waypoint<'a>, error::Error> {
        let mut arguments = Vec::new();
        let result = self.conn.execute_procedure("", "", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Waypoint{id: return_value, conn: &self.conn})
    }

    // getters
    pub async fn get_waypoints(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "WaypointManager_get_Waypoints", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_icons(&'a self) -> Result<(/*list*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "WaypointManager_get_Icons", arguments).await?;
        let return_value = decoder::decode_list(result)?;
        Ok(return_value)
    }

    pub async fn get_colors(&'a self) -> Result<(/*dict*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "WaypointManager_get_Colors", arguments).await?;
        let return_value = decoder::decode_dictionary(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

#[derive(Debug)]
pub struct Wheel<'a> {
    id: u64,
    conn: &'a Connection,
}
impl<'a> Wheel<'a> {
    // methods
    // getters
    pub async fn get_part(&'a self) -> Result<Part<'a>, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Part", arguments).await?;
        let return_value = decoder::decode_class(result)?;
        Ok(Part{id: return_value, conn: &self.conn})
    }

    pub async fn get_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_State", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_radius(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Radius", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_grounded(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Grounded", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_has_brakes(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_HasBrakes", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_brakes(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Brakes", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_brakes(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_Brakes", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_auto_friction_control(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_AutoFrictionControl", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_auto_friction_control(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_AutoFrictionControl", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_manual_friction_control(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_ManualFrictionControl", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_manual_friction_control(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_ManualFrictionControl", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_deployable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Deployable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_deployed(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Deployed", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_deployed(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_Deployed", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_powered(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Powered", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_motor_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_MotorEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_motor_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_MotorEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_motor_inverted(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_MotorInverted", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_motor_inverted(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_MotorInverted", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_motor_state(&'a self) -> Result<(/*enum*/), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_MotorState", arguments).await?;
        let return_value = decoder::decode_enumeration(result)?;
        Ok(return_value)
    }

    pub async fn get_motor_output(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_MotorOutput", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_traction_control_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_TractionControlEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_traction_control_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_TractionControlEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_traction_control(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_TractionControl", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_traction_control(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_TractionControl", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_drive_limiter(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_DriveLimiter", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn set_drive_limiter(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_float(0.0)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_DriveLimiter", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_steerable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Steerable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_steering_enabled(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_SteeringEnabled", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_steering_enabled(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_SteeringEnabled", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_steering_inverted(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_SteeringInverted", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn set_steering_inverted(&'a self) -> Result<(), error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        arguments.push(schema::Argument {
            position: 1,
            value: encoder::encode_bool()?,
        });
        let result = self.conn.execute_procedure("", "Wheel_set_SteeringInverted", arguments).await?;
        let return_value = decoder::decode_none(result)?;
        Ok(())
    }

    pub async fn get_has_suspension(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_HasSuspension", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_suspension_spring_strength(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_SuspensionSpringStrength", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_suspension_damper_strength(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_SuspensionDamperStrength", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_broken(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Broken", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_repairable(&'a self) -> Result<bool, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Repairable", arguments).await?;
        let return_value = decoder::decode_bool(result)?;
        Ok(return_value)
    }

    pub async fn get_stress(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Stress", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_stress_tolerance(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_StressTolerance", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_stress_percentage(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_StressPercentage", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_deflection(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Deflection", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    pub async fn get_slip(&'a self) -> Result<f32, error::Error> {
        let mut arguments = Vec::new();
        arguments.push(schema::Argument {
            position: 0,
            value: encoder::encode_u64(self.id)?,
        });
        let result = self.conn.execute_procedure("", "Wheel_get_Slip", arguments).await?;
        let return_value = decoder::decode_float(result)?;
        Ok(return_value)
    }

    
    // setters
    
    // static methods
}

