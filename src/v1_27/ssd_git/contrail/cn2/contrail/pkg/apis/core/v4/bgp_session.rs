// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSession

/// BGPSession defines the attributes for 1 (typically) or more sessions between two BGP Routers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPSession {
    /// There can be up to 3 instances BGP session attributes, representing configuration for both ends and common. Currently, only 1 instance representing common attributes is supported.
    pub session_attributes: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSessionAttributes>>,
}

impl crate::DeepMerge for BGPSession {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.session_attributes, other.session_attributes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for BGPSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_session_attributes,
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
                            "sessionAttributes" => Field::Key_session_attributes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = BGPSession;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPSession")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_session_attributes: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSessionAttributes>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_session_attributes => value_session_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPSession {
                    session_attributes: value_session_attributes,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPSession",
            &[
                "sessionAttributes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for BGPSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPSession",
            self.session_attributes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.session_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sessionAttributes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPSession {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSession".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPSession defines the attributes for 1 (typically) or more sessions between two BGP Routers.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "sessionAttributes".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("There can be up to 3 instances BGP session attributes, representing configuration for both ends and common. Currently, only 1 instance representing common attributes is supported.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::BGPSessionAttributes>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
