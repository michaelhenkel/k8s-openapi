// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairs

/// AllowedAddressPairs is a list of AllowedAddressPair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AllowedAddressPairs {
    pub allowed_address_pair: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPair>>,
}

impl crate::DeepMerge for AllowedAddressPairs {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.allowed_address_pair, other.allowed_address_pair);
    }
}

impl<'de> crate::serde::Deserialize<'de> for AllowedAddressPairs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allowed_address_pair,
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
                            "allowedAddressPair" => Field::Key_allowed_address_pair,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = AllowedAddressPairs;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AllowedAddressPairs")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_allowed_address_pair: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPair>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allowed_address_pair => value_allowed_address_pair = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AllowedAddressPairs {
                    allowed_address_pair: value_allowed_address_pair,
                })
            }
        }

        deserializer.deserialize_struct(
            "AllowedAddressPairs",
            &[
                "allowedAddressPair",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for AllowedAddressPairs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AllowedAddressPairs",
            self.allowed_address_pair.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allowed_address_pair {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "allowedAddressPair", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for AllowedAddressPairs {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.AllowedAddressPairs".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("AllowedAddressPairs is a list of AllowedAddressPair.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "allowedAddressPair".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::AllowedAddressPair>()))),
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
