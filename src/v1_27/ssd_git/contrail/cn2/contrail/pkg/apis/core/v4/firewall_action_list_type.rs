// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallActionListType

/// FirewallActionListType defines types of actions performed by FirewallRule.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FirewallActionListType {
    pub mirror_to: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMirrorActionType>,

    /// SimpleAction defines allow(i.e. pass) or deny action for traffic matching this FirewallRule.
    pub simple_action: Option<String>,
}

impl crate::DeepMerge for FirewallActionListType {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.mirror_to, other.mirror_to);
        crate::DeepMerge::merge_from(&mut self.simple_action, other.simple_action);
    }
}

impl<'de> crate::serde::Deserialize<'de> for FirewallActionListType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_mirror_to,
            Key_simple_action,
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
                            "mirrorTo" => Field::Key_mirror_to,
                            "simpleAction" => Field::Key_simple_action,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallActionListType;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("FirewallActionListType")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_mirror_to: Option<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMirrorActionType> = None;
                let mut value_simple_action: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_mirror_to => value_mirror_to = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_simple_action => value_simple_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(FirewallActionListType {
                    mirror_to: value_mirror_to,
                    simple_action: value_simple_action,
                })
            }
        }

        deserializer.deserialize_struct(
            "FirewallActionListType",
            &[
                "mirrorTo",
                "simpleAction",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for FirewallActionListType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "FirewallActionListType",
            self.mirror_to.as_ref().map_or(0, |_| 1) +
            self.simple_action.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.mirror_to {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "mirrorTo", value)?;
        }
        if let Some(value) = &self.simple_action {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "simpleAction", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for FirewallActionListType {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.FirewallActionListType".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("FirewallActionListType defines types of actions performed by FirewallRule.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "mirrorTo".to_owned(),
                        __gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::FirewallMirrorActionType>(),
                    ),
                    (
                        "simpleAction".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SimpleAction defines allow(i.e. pass) or deny action for traffic matching this FirewallRule.".to_owned()),
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
