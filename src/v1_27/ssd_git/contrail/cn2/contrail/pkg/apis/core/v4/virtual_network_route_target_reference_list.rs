// Generated from definition net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouteTargetReferenceList

/// VirtualNetworkRouteTargetReferenceList contains RouteTarget references and their import/export mode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualNetworkRouteTargetReferenceList {
    /// RouteTargetReferences is the actual list of RouteTargetReferences.
    pub route_target_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>>,
}

impl crate::DeepMerge for VirtualNetworkRouteTargetReferenceList {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.route_target_references, other.route_target_references);
    }
}

impl<'de> crate::serde::Deserialize<'de> for VirtualNetworkRouteTargetReferenceList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_route_target_references,
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
                            "routeTargetReferences" => Field::Key_route_target_references,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = VirtualNetworkRouteTargetReferenceList;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("VirtualNetworkRouteTargetReferenceList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_route_target_references: Option<Vec<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_route_target_references => value_route_target_references = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(VirtualNetworkRouteTargetReferenceList {
                    route_target_references: value_route_target_references,
                })
            }
        }

        deserializer.deserialize_struct(
            "VirtualNetworkRouteTargetReferenceList",
            &[
                "routeTargetReferences",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for VirtualNetworkRouteTargetReferenceList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "VirtualNetworkRouteTargetReferenceList",
            self.route_target_references.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.route_target_references {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "routeTargetReferences", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for VirtualNetworkRouteTargetReferenceList {
    fn schema_name() -> String {
        "net.juniper.ssd-git.contrail.cn2.contrail.pkg.apis.core.v4.VirtualNetworkRouteTargetReferenceList".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("VirtualNetworkRouteTargetReferenceList contains RouteTarget references and their import/export mode.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "routeTargetReferences".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("RouteTargetReferences is the actual list of RouteTargetReferences.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::ssd_git::contrail::cn2::contrail::pkg::apis::core::v4::RouteTargetReference>()))),
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
