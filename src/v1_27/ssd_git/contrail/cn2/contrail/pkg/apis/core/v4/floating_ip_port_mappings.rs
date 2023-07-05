// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortMappings

/// FloatingIPPortMappings is the list port mappings associated with a given FloatingIP.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FloatingIPPortMappings {
    /// PortMappings is a list of FloatingIPPortPortMapping instances.
    pub port_mappings: Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FloatingIPPortPortMapping>,
}

impl crate::DeepMerge for FloatingIPPortMappings {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.port_mappings, other.port_mappings);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FloatingIPPortMappings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_port_mappings,
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
                            "portMappings" => Field::Key_port_mappings,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FloatingIPPortMappings;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FloatingIPPortMappings")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_port_mappings: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FloatingIPPortPortMapping>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_port_mappings => value_port_mappings = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FloatingIPPortMappings {
                    port_mappings: value_port_mappings.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "FloatingIPPortMappings",
            &[
                "portMappings",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FloatingIPPortMappings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FloatingIPPortMappings",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "portMappings", &self.port_mappings)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FloatingIPPortMappings {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FloatingIPPortMappings".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FloatingIPPortMappings is the list port mappings associated with a given FloatingIP.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "portMappings".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PortMappings is a list of FloatingIPPortPortMapping instances.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FloatingIPPortPortMapping>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "portMappings".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
