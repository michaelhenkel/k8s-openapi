// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallSubnet

/// FirewallSubnet defines the IP prefix and length.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallSubnet {
    /// Network prefix
    pub ip_prefix: Option<String>,

    /// Network prefix length
    pub ip_prefix_len: Option<crate::apimachinery::pkg::util::intstr::IntOrString>,
}

impl crate::DeepMerge for FirewallSubnet {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.ip_prefix, other.ip_prefix);
        crate::DeepMerge::merge_from(&mut self.ip_prefix_len, other.ip_prefix_len);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FirewallSubnet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_ip_prefix,
            Key_ip_prefix_len,
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
                            "ipPrefix" => Field::Key_ip_prefix,
                            "ipPrefixLen" => Field::Key_ip_prefix_len,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallSubnet;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallSubnet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_ip_prefix: Option<String> = None;
                let mut value_ip_prefix_len: Option<crate::apimachinery::pkg::util::intstr::IntOrString> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_ip_prefix => value_ip_prefix = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ip_prefix_len => value_ip_prefix_len = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallSubnet {
                    ip_prefix: value_ip_prefix,
                    ip_prefix_len: value_ip_prefix_len,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallSubnet",
            &[
                "ipPrefix",
                "ipPrefixLen",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FirewallSubnet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallSubnet",
            self.ip_prefix.as_ref().map_or(0, |_| 1) +
            self.ip_prefix_len.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.ip_prefix {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipPrefix", value)?;
        }
        if let Some(value) = &self.ip_prefix_len {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ipPrefixLen", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallSubnet {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallSubnet".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FirewallSubnet defines the IP prefix and length.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "ipPrefix".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Network prefix".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "ipPrefixLen".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::util::intstr::IntOrString>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Network prefix length".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
