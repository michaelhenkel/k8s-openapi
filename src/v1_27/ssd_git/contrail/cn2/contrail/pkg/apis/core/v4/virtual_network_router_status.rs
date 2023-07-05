// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterStatus

/// VirtualNetworkRouterStatus defines the observed state of the VirtualNetworkRouter.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualNetworkRouterStatus {
    /// EvpnRoutingVirtualNetworkReference is the reference to VirtualNetworkRouter's  internal Virtual Network
    pub evpn_routing_virtual_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>,

    /// L3vxlanNetworkIdentifier is L3 VNI associated with internal network of VirtualNetworkRouter in case EvpnRouting is enabled.
    pub l3vxlan_network_identifier: Option<i64>,

    /// Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.
    pub observation: String,

    /// State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.
    pub state: String,
}

impl crate::DeepMerge for VirtualNetworkRouterStatus {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.evpn_routing_virtual_network_reference, other.evpn_routing_virtual_network_reference);
        crate::DeepMerge::merge_from(&mut self.l3vxlan_network_identifier, other.l3vxlan_network_identifier);
        crate::DeepMerge::merge_from(&mut self.observation, other.observation);
        crate::DeepMerge::merge_from(&mut self.state, other.state);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VirtualNetworkRouterStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_evpn_routing_virtual_network_reference,
            Key_l3vxlan_network_identifier,
            Key_observation,
            Key_state,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "evpnRoutingVirtualNetworkReference" => Field::Key_evpn_routing_virtual_network_reference,
                            "l3vxlanNetworkIdentifier" => Field::Key_l3vxlan_network_identifier,
                            "observation" => Field::Key_observation,
                            "state" => Field::Key_state,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkRouterStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkRouterStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_evpn_routing_virtual_network_reference: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference> = None;
                let mut value_l3vxlan_network_identifier: Option<i64> = None;
                let mut value_observation: Option<String> = None;
                let mut value_state: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_evpn_routing_virtual_network_reference => value_evpn_routing_virtual_network_reference = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_l3vxlan_network_identifier => value_l3vxlan_network_identifier = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_observation => value_observation = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_state => value_state = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkRouterStatus {
                    evpn_routing_virtual_network_reference: value_evpn_routing_virtual_network_reference,
                    l3vxlan_network_identifier: value_l3vxlan_network_identifier,
                    observation: value_observation.unwrap_or_default(),
                    state: value_state.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkRouterStatus",
            &[
                "evpnRoutingVirtualNetworkReference",
                "l3vxlanNetworkIdentifier",
                "observation",
                "state",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VirtualNetworkRouterStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkRouterStatus",
            2 +
            self.evpn_routing_virtual_network_reference.as_ref().map_or(0, |_| 1) +
            self.l3vxlan_network_identifier.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.evpn_routing_virtual_network_reference {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "evpnRoutingVirtualNetworkReference", value)?;
        }
        if let Some(value) = &self.l3vxlan_network_identifier {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "l3vxlanNetworkIdentifier", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "observation", &self.observation)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "state", &self.state)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkRouterStatus {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouterStatus".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualNetworkRouterStatus defines the observed state of the VirtualNetworkRouter.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "evpnRoutingVirtualNetworkReference".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::ResourceReference>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("EvpnRoutingVirtualNetworkReference is the reference to VirtualNetworkRouter's  internal Virtual Network".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "l3vxlanNetworkIdentifier".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("L3vxlanNetworkIdentifier is L3 VNI associated with internal network of VirtualNetworkRouter in case EvpnRouting is enabled.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "observation".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Observation provides additional information related to the state of the resource. For example, if a reconciliation error occurs, Observation will contain a brief description of the problem.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "state".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("State describe the current readiness of a resource after the last reconciliation. The possible states include Pending, Success, and Failure.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "observation".to_owned(),
                    "state".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
