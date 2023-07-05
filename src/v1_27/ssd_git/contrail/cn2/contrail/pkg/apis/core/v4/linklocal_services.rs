// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServices

/// LinklocalServices is list of LinkLocalServices
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinklocalServices {
    pub linklocal_service_entry: Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::LinklocalServiceEntryType>,
}

impl crate::DeepMerge for LinklocalServices {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.linklocal_service_entry, other.linklocal_service_entry);
    }
}

impl<'de> crate::serde::Deserialize<'de> for LinklocalServices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_linklocal_service_entry,
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
                            "linklocalServiceEntry" => Field::Key_linklocal_service_entry,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = LinklocalServices;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("LinklocalServices")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_linklocal_service_entry: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::LinklocalServiceEntryType>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_linklocal_service_entry => value_linklocal_service_entry = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(LinklocalServices {
                    linklocal_service_entry: value_linklocal_service_entry.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "LinklocalServices",
            &[
                "linklocalServiceEntry",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for LinklocalServices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "LinklocalServices",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "linklocalServiceEntry", &self.linklocal_service_entry)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for LinklocalServices {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.LinklocalServices".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("LinklocalServices is list of LinkLocalServices".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "linklocalServiceEntry".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::LinklocalServiceEntryType>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "linklocalServiceEntry".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
