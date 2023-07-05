// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPool

/// PortTranslationPool represents a range or quantity of available ports for a given protocol. Protocol is required. PortRange is optional defaults to nil. PortCount is optional defaults to 0. Only one of PortRange and PortCount may be set for the PortTranslationPool to be valid.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PortTranslationPool {
    /// PortCount defines the maximum amount of port numbers to be allocated. If PortCount is greater than 0, PortRange must be empty.
    pub port_count: Option<i32>,

    /// PortRange defines the range from which port numbers may be allocated. If PortRange is set, PortCount must be 0.
    pub port_range: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortRange>,

    /// Protocol specifies the protocol this pools is for.
    pub protocol: String,
}

impl crate::DeepMerge for PortTranslationPool {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.port_count, other.port_count);
        crate::DeepMerge::merge_from(&mut self.port_range, other.port_range);
        crate::DeepMerge::merge_from(&mut self.protocol, other.protocol);
    }
}

impl<'de> crate::serde::Deserialize<'de> for PortTranslationPool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_port_count,
            Key_port_range,
            Key_protocol,
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
                            "portCount" => Field::Key_port_count,
                            "portRange" => Field::Key_port_range,
                            "protocol" => Field::Key_protocol,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = PortTranslationPool;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PortTranslationPool")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_port_count: Option<i32> = None;
                let mut value_port_range: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortRange> = None;
                let mut value_protocol: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_port_count => value_port_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_port_range => value_port_range = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_protocol => value_protocol = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PortTranslationPool {
                    port_count: value_port_count,
                    port_range: value_port_range,
                    protocol: value_protocol.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "PortTranslationPool",
            &[
                "portCount",
                "portRange",
                "protocol",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for PortTranslationPool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PortTranslationPool",
            1 +
            self.port_count.as_ref().map_or(0, |_| 1) +
            self.port_range.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.port_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portCount", value)?;
        }
        if let Some(value) = &self.port_range {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portRange", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "protocol", &self.protocol)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for PortTranslationPool {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.PortTranslationPool".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("PortTranslationPool represents a range or quantity of available ports for a given protocol. Protocol is required. PortRange is optional defaults to nil. PortCount is optional defaults to 0. Only one of PortRange and PortCount may be set for the PortTranslationPool to be valid.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "portCount".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PortCount defines the maximum amount of port numbers to be allocated. If PortCount is greater than 0, PortRange must be empty.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "portRange".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::PortRange>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PortRange defines the range from which port numbers may be allocated. If PortRange is set, PortCount must be 0.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "protocol".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Protocol specifies the protocol this pools is for.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "protocol".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
