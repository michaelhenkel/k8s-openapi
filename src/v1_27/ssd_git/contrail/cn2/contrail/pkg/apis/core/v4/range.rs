// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.Range

/// Range is a list of IPRanges associated with a given key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Range {
    /// IPRanges lists one or more IPRange instance.
    pub ip_ranges: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::IPRange>>,

    /// Key is a text string defining the Range collection. Setting a Range with an existing key will overwrite the exiting Range.
    pub key: Option<String>,
}

impl crate::DeepMerge for Range {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.ip_ranges, other.ip_ranges);
        crate::DeepMerge::merge_from(&mut self.key, other.key);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip_ranges,
            Key_key,
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
                            "ipRanges" => Field::Key_ip_ranges,
                            "key" => Field::Key_key,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Range;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Range")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ip_ranges: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::IPRange>> = None;
                let mut value_key: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip_ranges => value_ip_ranges = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_key => value_key = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Range {
                    ip_ranges: value_ip_ranges,
                    key: value_key,
                })
            }
        }

        deserializer.deserialize_struct(
            "Range",
            &[
                "ipRanges",
                "key",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Range",
            self.ip_ranges.as_ref().map_or(0, |_| 1) +
            self.key.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip_ranges {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipRanges", value)?;
        }
        if let Some(value) = &self.key {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "key", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Range {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.Range".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Range is a list of IPRanges associated with a given key.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ipRanges".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("IPRanges lists one or more IPRange instance.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::IPRange>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "key".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Key is a text string defining the Range collection. Setting a Range with an existing key will overwrite the exiting Range.".to_owned()),
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
