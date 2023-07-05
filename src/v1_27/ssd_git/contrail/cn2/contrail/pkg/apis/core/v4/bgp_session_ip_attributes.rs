// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes

/// BGPSessionIPAttributes contains BGPSession primary and secondary IP addresses.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BGPSessionIPAttributes {
    /// BGPaaSPrimaryIP defines the primary IP address used for a BGP session.
    pub bgpaas_primary_ip: Option<String>,

    /// BGPaaSSecondaryIP defines the secondary IP address used for a BGP session when a second control node is present.
    pub bgpaas_secondary_ip: Option<String>,
}

impl crate::DeepMerge for BGPSessionIPAttributes {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.bgpaas_primary_ip, other.bgpaas_primary_ip);
        crate::DeepMerge::merge_from(&mut self.bgpaas_secondary_ip, other.bgpaas_secondary_ip);
    }
}

impl<'de> crate::serde::Deserialize<'de> for BGPSessionIPAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bgpaas_primary_ip,
            Key_bgpaas_secondary_ip,
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
                            "bgpaasPrimaryIP" => Field::Key_bgpaas_primary_ip,
                            "bgpaasSecondaryIP" => Field::Key_bgpaas_secondary_ip,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = BGPSessionIPAttributes;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BGPSessionIPAttributes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_bgpaas_primary_ip: Option<String> = None;
                let mut value_bgpaas_secondary_ip: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bgpaas_primary_ip => value_bgpaas_primary_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bgpaas_secondary_ip => value_bgpaas_secondary_ip = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BGPSessionIPAttributes {
                    bgpaas_primary_ip: value_bgpaas_primary_ip,
                    bgpaas_secondary_ip: value_bgpaas_secondary_ip,
                })
            }
        }

        deserializer.deserialize_struct(
            "BGPSessionIPAttributes",
            &[
                "bgpaasPrimaryIP",
                "bgpaasSecondaryIP",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for BGPSessionIPAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BGPSessionIPAttributes",
            self.bgpaas_primary_ip.as_ref().map_or(0, |_| 1) +
            self.bgpaas_secondary_ip.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bgpaas_primary_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpaasPrimaryIP", value)?;
        }
        if let Some(value) = &self.bgpaas_secondary_ip {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bgpaasSecondaryIP", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for BGPSessionIPAttributes {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.BGPSessionIPAttributes".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("BGPSessionIPAttributes contains BGPSession primary and secondary IP addresses.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bgpaasPrimaryIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPaaSPrimaryIP defines the primary IP address used for a BGP session.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "bgpaasSecondaryIP".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("BGPaaSSecondaryIP defines the secondary IP address used for a BGP session when a second control node is present.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
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
