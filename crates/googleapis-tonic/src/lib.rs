pub mod cloud {
    pub mod kubernetes {
        pub mod security {
            pub mod containersecurity_logging {
                include!("cloud.kubernetes.security.containersecurity_logging.rs");
            }
        }
    }
}
pub mod google {
    pub mod actions {
        pub mod r#type {
            include!("google.actions.r#type.rs");
        }
        pub mod sdk {
            pub mod v2 {
                include!("google.actions.sdk.v2.rs");
                pub mod conversation {
                    include!("google.actions.sdk.v2.conversation.rs");
                }
                pub mod interactionmodel {
                    include!("google.actions.sdk.v2.interactionmodel.rs");
                    pub mod prompt {
                        include!("google.actions.sdk.v2.interactionmodel.prompt.rs");
                    }
                    pub mod r#type {
                        include!("google.actions.sdk.v2.interactionmodel.r#type.rs");
                    }
                }
            }
        }
    }
    pub mod ads {
        pub mod admanager {
            pub mod v1 {
                include!("google.ads.admanager.v1.rs");
            }
        }
        pub mod admob {
            pub mod v1 {
                include!("google.ads.admob.v1.rs");
            }
        }
        pub mod googleads {
            pub mod v15 {
                pub mod common {
                    include!("google.ads.googleads.v15.common.rs");
                }
                pub mod enums {
                    include!("google.ads.googleads.v15.enums.rs");
                }
                pub mod errors {
                    include!("google.ads.googleads.v15.errors.rs");
                }
                pub mod resources {
                    include!("google.ads.googleads.v15.resources.rs");
                }
                pub mod services {
                    include!("google.ads.googleads.v15.services.rs");
                }
            }
            pub mod v16 {
                pub mod common {
                    include!("google.ads.googleads.v16.common.rs");
                }
                pub mod enums {
                    include!("google.ads.googleads.v16.enums.rs");
                }
                pub mod errors {
                    include!("google.ads.googleads.v16.errors.rs");
                }
                pub mod resources {
                    include!("google.ads.googleads.v16.resources.rs");
                }
                pub mod services {
                    include!("google.ads.googleads.v16.services.rs");
                }
            }
            pub mod v17 {
                pub mod common {
                    include!("google.ads.googleads.v17.common.rs");
                }
                pub mod enums {
                    include!("google.ads.googleads.v17.enums.rs");
                }
                pub mod errors {
                    include!("google.ads.googleads.v17.errors.rs");
                }
                pub mod resources {
                    include!("google.ads.googleads.v17.resources.rs");
                }
                pub mod services {
                    include!("google.ads.googleads.v17.services.rs");
                }
            }
        }
        pub mod searchads360 {
            pub mod v0 {
                pub mod common {
                    include!("google.ads.searchads360.v0.common.rs");
                }
                pub mod enums {
                    include!("google.ads.searchads360.v0.enums.rs");
                }
                pub mod errors {
                    include!("google.ads.searchads360.v0.errors.rs");
                }
                pub mod resources {
                    include!("google.ads.searchads360.v0.resources.rs");
                }
                pub mod services {
                    include!("google.ads.searchads360.v0.services.rs");
                }
            }
        }
    }
    pub mod ai {
        pub mod generativelanguage {
            pub mod v1 {
                include!("google.ai.generativelanguage.v1.rs");
            }
            pub mod v1beta {
                include!("google.ai.generativelanguage.v1beta.rs");
            }
            pub mod v1beta2 {
                include!("google.ai.generativelanguage.v1beta2.rs");
            }
            pub mod v1beta3 {
                include!("google.ai.generativelanguage.v1beta3.rs");
            }
        }
    }
    pub mod analytics {
        pub mod admin {
            pub mod v1alpha {
                include!("google.analytics.admin.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.analytics.admin.v1beta.rs");
            }
        }
        pub mod data {
            pub mod v1alpha {
                include!("google.analytics.data.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.analytics.data.v1beta.rs");
            }
        }
    }
    pub mod api {
        include!("google.api.rs");
        pub mod apikeys {
            pub mod v2 {
                include!("google.api.apikeys.v2.rs");
            }
        }
        pub mod cloudquotas {
            pub mod v1 {
                include!("google.api.cloudquotas.v1.rs");
            }
        }
        pub mod expr {
            pub mod conformance {
                pub mod v1alpha1 {
                    include!("google.api.expr.conformance.v1alpha1.rs");
                }
            }
            pub mod v1alpha1 {
                include!("google.api.expr.v1alpha1.rs");
            }
            pub mod v1beta1 {
                include!("google.api.expr.v1beta1.rs");
            }
        }
        pub mod servicecontrol {
            pub mod v1 {
                include!("google.api.servicecontrol.v1.rs");
            }
            pub mod v2 {
                include!("google.api.servicecontrol.v2.rs");
            }
        }
        pub mod servicemanagement {
            pub mod v1 {
                include!("google.api.servicemanagement.v1.rs");
            }
        }
        pub mod serviceusage {
            pub mod v1 {
                include!("google.api.serviceusage.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.api.serviceusage.v1beta1.rs");
            }
        }
    }
    pub mod appengine {
        pub mod legacy {
            include!("google.appengine.legacy.rs");
        }
        pub mod logging {
            pub mod v1 {
                include!("google.appengine.logging.v1.rs");
            }
        }
        pub mod v1 {
            include!("google.appengine.v1.rs");
        }
        pub mod v1beta {
            include!("google.appengine.v1beta.rs");
        }
    }
    pub mod apps {
        pub mod alertcenter {
            pub mod v1beta1 {
                include!("google.apps.alertcenter.v1beta1.rs");
            }
        }
        pub mod card {
            pub mod v1 {
                include!("google.apps.card.v1.rs");
            }
        }
        pub mod drive {
            pub mod activity {
                pub mod v2 {
                    include!("google.apps.drive.activity.v2.rs");
                }
            }
            pub mod labels {
                pub mod v2 {
                    include!("google.apps.drive.labels.v2.rs");
                }
                pub mod v2beta {
                    include!("google.apps.drive.labels.v2beta.rs");
                }
            }
        }
        pub mod events {
            pub mod subscriptions {
                pub mod v1 {
                    include!("google.apps.events.subscriptions.v1.rs");
                }
            }
        }
        pub mod meet {
            pub mod v2 {
                include!("google.apps.meet.v2.rs");
            }
            pub mod v2beta {
                include!("google.apps.meet.v2beta.rs");
            }
        }
        pub mod script {
            pub mod r#type {
                include!("google.apps.script.r#type.rs");
                pub mod calendar {
                    include!("google.apps.script.r#type.calendar.rs");
                }
                pub mod docs {
                    include!("google.apps.script.r#type.docs.rs");
                }
                pub mod drive {
                    include!("google.apps.script.r#type.drive.rs");
                }
                pub mod gmail {
                    include!("google.apps.script.r#type.gmail.rs");
                }
                pub mod sheets {
                    include!("google.apps.script.r#type.sheets.rs");
                }
                pub mod slides {
                    include!("google.apps.script.r#type.slides.rs");
                }
            }
        }
    }
    pub mod area120 {
        pub mod tables {
            pub mod v1alpha1 {
                include!("google.area120.tables.v1alpha1.rs");
            }
        }
    }
    pub mod assistant {
        pub mod embedded {
            pub mod v1alpha1 {
                include!("google.assistant.embedded.v1alpha1.rs");
            }
            pub mod v1alpha2 {
                include!("google.assistant.embedded.v1alpha2.rs");
            }
        }
    }
    pub mod bigtable {
        pub mod admin {
            pub mod v2 {
                include!("google.bigtable.admin.v2.rs");
            }
        }
        pub mod v2 {
            include!("google.bigtable.v2.rs");
        }
    }
    pub mod bytestream {
        include!("google.bytestream.rs");
    }
    pub mod chat {
        pub mod logging {
            pub mod v1 {
                include!("google.chat.logging.v1.rs");
            }
        }
        pub mod v1 {
            include!("google.chat.v1.rs");
        }
    }
    pub mod chromeos {
        pub mod moblab {
            pub mod v1beta1 {
                include!("google.chromeos.moblab.v1beta1.rs");
            }
        }
        pub mod uidetection {
            pub mod v1 {
                include!("google.chromeos.uidetection.v1.rs");
            }
        }
    }
    pub mod cloud {
        include!("google.cloud.rs");
        pub mod abuseevent {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.abuseevent.logging.v1.rs");
                }
            }
        }
        pub mod accessapproval {
            pub mod v1 {
                include!("google.cloud.accessapproval.v1.rs");
            }
        }
        pub mod advisorynotifications {
            pub mod v1 {
                include!("google.cloud.advisorynotifications.v1.rs");
            }
        }
        pub mod aiplatform {
            pub mod logging {
                include!("google.cloud.aiplatform.logging.rs");
            }
            pub mod v1 {
                include!("google.cloud.aiplatform.v1.rs");
                pub mod schema {
                    pub mod predict {
                        pub mod instance {
                            include!("google.cloud.aiplatform.v1.schema.predict.instance.rs");
                        }
                        pub mod params {
                            include!("google.cloud.aiplatform.v1.schema.predict.params.rs");
                        }
                        pub mod prediction {
                            include!("google.cloud.aiplatform.v1.schema.predict.prediction.rs");
                        }
                    }
                    pub mod trainingjob {
                        pub mod definition {
                            include!("google.cloud.aiplatform.v1.schema.trainingjob.definition.rs");
                        }
                    }
                }
            }
            pub mod v1beta1 {
                include!("google.cloud.aiplatform.v1beta1.rs");
                pub mod schema {
                    include!("google.cloud.aiplatform.v1beta1.schema.rs");
                    pub mod predict {
                        pub mod instance {
                            include!("google.cloud.aiplatform.v1beta1.schema.predict.instance.rs");
                        }
                        pub mod params {
                            include!("google.cloud.aiplatform.v1beta1.schema.predict.params.rs");
                        }
                        pub mod prediction {
                            include!(
                                "google.cloud.aiplatform.v1beta1.schema.predict.prediction.rs"
                            );
                        }
                    }
                    pub mod trainingjob {
                        pub mod definition {
                            include!(
                                "google.cloud.aiplatform.v1beta1.schema.trainingjob.definition.rs"
                            );
                        }
                    }
                }
            }
        }
        pub mod alloydb {
            pub mod connectors {
                pub mod v1 {
                    include!("google.cloud.alloydb.connectors.v1.rs");
                }
                pub mod v1alpha {
                    include!("google.cloud.alloydb.connectors.v1alpha.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.alloydb.connectors.v1beta.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.alloydb.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.alloydb.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.alloydb.v1beta.rs");
            }
        }
        pub mod apigateway {
            pub mod v1 {
                include!("google.cloud.apigateway.v1.rs");
            }
        }
        pub mod apigeeconnect {
            pub mod v1 {
                include!("google.cloud.apigeeconnect.v1.rs");
            }
        }
        pub mod apigeeregistry {
            pub mod v1 {
                include!("google.cloud.apigeeregistry.v1.rs");
            }
        }
        pub mod apphub {
            pub mod v1 {
                include!("google.cloud.apphub.v1.rs");
            }
        }
        pub mod asset {
            pub mod v1 {
                include!("google.cloud.asset.v1.rs");
            }
            pub mod v1p1beta1 {
                include!("google.cloud.asset.v1p1beta1.rs");
            }
            pub mod v1p2beta1 {
                include!("google.cloud.asset.v1p2beta1.rs");
            }
            pub mod v1p5beta1 {
                include!("google.cloud.asset.v1p5beta1.rs");
            }
            pub mod v1p7beta1 {
                include!("google.cloud.asset.v1p7beta1.rs");
            }
        }
        pub mod assuredworkloads {
            pub mod regulatoryintercept {
                pub mod logging {
                    pub mod v1 {
                        include!("google.cloud.assuredworkloads.regulatoryintercept.logging.v1.rs");
                    }
                }
            }
            pub mod v1 {
                include!("google.cloud.assuredworkloads.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.assuredworkloads.v1beta1.rs");
            }
        }
        pub mod audit {
            include!("google.cloud.audit.rs");
        }
        pub mod automl {
            pub mod v1 {
                include!("google.cloud.automl.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.automl.v1beta1.rs");
            }
        }
        pub mod backupdr {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.backupdr.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.backupdr.v1.rs");
            }
        }
        pub mod baremetalsolution {
            pub mod v2 {
                include!("google.cloud.baremetalsolution.v2.rs");
            }
        }
        pub mod batch {
            pub mod v1 {
                include!("google.cloud.batch.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.batch.v1alpha.rs");
            }
        }
        pub mod beyondcorp {
            pub mod appconnections {
                pub mod v1 {
                    include!("google.cloud.beyondcorp.appconnections.v1.rs");
                }
            }
            pub mod appconnectors {
                pub mod v1 {
                    include!("google.cloud.beyondcorp.appconnectors.v1.rs");
                }
            }
            pub mod appgateways {
                pub mod v1 {
                    include!("google.cloud.beyondcorp.appgateways.v1.rs");
                }
            }
            pub mod clientconnectorservices {
                pub mod v1 {
                    include!("google.cloud.beyondcorp.clientconnectorservices.v1.rs");
                }
            }
            pub mod clientgateways {
                pub mod v1 {
                    include!("google.cloud.beyondcorp.clientgateways.v1.rs");
                }
            }
        }
        pub mod bigquery {
            pub mod analyticshub {
                pub mod v1 {
                    include!("google.cloud.bigquery.analyticshub.v1.rs");
                }
            }
            pub mod biglake {
                pub mod v1 {
                    include!("google.cloud.bigquery.biglake.v1.rs");
                }
                pub mod v1alpha1 {
                    include!("google.cloud.bigquery.biglake.v1alpha1.rs");
                }
            }
            pub mod connection {
                pub mod v1 {
                    include!("google.cloud.bigquery.connection.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.bigquery.connection.v1beta1.rs");
                }
            }
            pub mod dataexchange {
                pub mod v1beta1 {
                    include!("google.cloud.bigquery.dataexchange.v1beta1.rs");
                }
            }
            pub mod datapolicies {
                pub mod v1 {
                    include!("google.cloud.bigquery.datapolicies.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.bigquery.datapolicies.v1beta1.rs");
                }
            }
            pub mod datatransfer {
                pub mod v1 {
                    include!("google.cloud.bigquery.datatransfer.v1.rs");
                }
            }
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.bigquery.logging.v1.rs");
                }
            }
            pub mod migration {
                pub mod v2 {
                    include!("google.cloud.bigquery.migration.v2.rs");
                }
                pub mod v2alpha {
                    include!("google.cloud.bigquery.migration.v2alpha.rs");
                }
            }
            pub mod reservation {
                pub mod v1 {
                    include!("google.cloud.bigquery.reservation.v1.rs");
                }
            }
            pub mod storage {
                pub mod v1 {
                    include!("google.cloud.bigquery.storage.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.bigquery.storage.v1beta1.rs");
                }
                pub mod v1beta2 {
                    include!("google.cloud.bigquery.storage.v1beta2.rs");
                }
            }
            pub mod v2 {
                include!("google.cloud.bigquery.v2.rs");
            }
        }
        pub mod billing {
            pub mod budgets {
                pub mod v1 {
                    include!("google.cloud.billing.budgets.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.billing.budgets.v1beta1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.billing.v1.rs");
            }
        }
        pub mod binaryauthorization {
            pub mod v1 {
                include!("google.cloud.binaryauthorization.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.binaryauthorization.v1beta1.rs");
            }
        }
        pub mod blockchainnodeengine {
            pub mod v1 {
                include!("google.cloud.blockchainnodeengine.v1.rs");
            }
        }
        pub mod certificatemanager {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.certificatemanager.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.certificatemanager.v1.rs");
            }
        }
        pub mod channel {
            pub mod v1 {
                include!("google.cloud.channel.v1.rs");
            }
        }
        pub mod cloudcontrolspartner {
            pub mod v1 {
                include!("google.cloud.cloudcontrolspartner.v1.rs");
            }
            pub mod v1beta {
                include!("google.cloud.cloudcontrolspartner.v1beta.rs");
            }
        }
        pub mod clouddms {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.clouddms.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.clouddms.v1.rs");
            }
        }
        pub mod cloudsetup {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.cloudsetup.logging.v1.rs");
                }
            }
        }
        pub mod commerce {
            pub mod consumer {
                pub mod procurement {
                    pub mod v1 {
                        include!("google.cloud.commerce.consumer.procurement.v1.rs");
                    }
                    pub mod v1alpha1 {
                        include!("google.cloud.commerce.consumer.procurement.v1alpha1.rs");
                    }
                }
            }
        }
        pub mod common {
            include!("google.cloud.common.rs");
        }
        pub mod compute {
            pub mod v1 {
                include!("google.cloud.compute.v1.rs");
            }
            pub mod v1small {
                include!("google.cloud.compute.v1small.rs");
            }
        }
        pub mod confidentialcomputing {
            pub mod v1 {
                include!("google.cloud.confidentialcomputing.v1.rs");
            }
            pub mod v1alpha1 {
                include!("google.cloud.confidentialcomputing.v1alpha1.rs");
            }
        }
        pub mod config {
            pub mod v1 {
                include!("google.cloud.config.v1.rs");
            }
        }
        pub mod connectors {
            pub mod v1 {
                include!("google.cloud.connectors.v1.rs");
            }
        }
        pub mod contactcenterinsights {
            pub mod v1 {
                include!("google.cloud.contactcenterinsights.v1.rs");
            }
        }
        pub mod contentwarehouse {
            pub mod v1 {
                include!("google.cloud.contentwarehouse.v1.rs");
            }
        }
        pub mod datacatalog {
            pub mod lineage {
                pub mod v1 {
                    include!("google.cloud.datacatalog.lineage.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.datacatalog.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.datacatalog.v1beta1.rs");
            }
        }
        pub mod dataform {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.dataform.logging.v1.rs");
                }
            }
            pub mod v1alpha2 {
                include!("google.cloud.dataform.v1alpha2.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.dataform.v1beta1.rs");
            }
        }
        pub mod datafusion {
            pub mod v1 {
                include!("google.cloud.datafusion.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.datafusion.v1beta1.rs");
            }
        }
        pub mod datalabeling {
            pub mod v1beta1 {
                include!("google.cloud.datalabeling.v1beta1.rs");
            }
        }
        pub mod datapipelines {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.datapipelines.logging.v1.rs");
                }
            }
        }
        pub mod dataplex {
            pub mod v1 {
                include!("google.cloud.dataplex.v1.rs");
            }
        }
        pub mod dataproc {
            pub mod logging {
                include!("google.cloud.dataproc.logging.rs");
            }
            pub mod v1 {
                include!("google.cloud.dataproc.v1.rs");
            }
        }
        pub mod dataqna {
            pub mod v1alpha {
                include!("google.cloud.dataqna.v1alpha.rs");
            }
        }
        pub mod datastream {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.datastream.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.datastream.v1.rs");
            }
            pub mod v1alpha1 {
                include!("google.cloud.datastream.v1alpha1.rs");
            }
        }
        pub mod deploy {
            pub mod v1 {
                include!("google.cloud.deploy.v1.rs");
            }
        }
        pub mod developerconnect {
            pub mod v1 {
                include!("google.cloud.developerconnect.v1.rs");
            }
        }
        pub mod dialogflow {
            pub mod cx {
                pub mod v3 {
                    include!("google.cloud.dialogflow.cx.v3.rs");
                }
                pub mod v3beta1 {
                    include!("google.cloud.dialogflow.cx.v3beta1.rs");
                }
            }
            pub mod v2 {
                include!("google.cloud.dialogflow.v2.rs");
            }
            pub mod v2beta1 {
                include!("google.cloud.dialogflow.v2beta1.rs");
            }
        }
        pub mod discoveryengine {
            pub mod logging {
                include!("google.cloud.discoveryengine.logging.rs");
            }
            pub mod v1 {
                include!("google.cloud.discoveryengine.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.discoveryengine.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.discoveryengine.v1beta.rs");
            }
        }
        pub mod documentai {
            pub mod v1 {
                include!("google.cloud.documentai.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.documentai.v1beta1.rs");
            }
            pub mod v1beta2 {
                include!("google.cloud.documentai.v1beta2.rs");
            }
            pub mod v1beta3 {
                include!("google.cloud.documentai.v1beta3.rs");
            }
        }
        pub mod domains {
            pub mod v1 {
                include!("google.cloud.domains.v1.rs");
            }
            pub mod v1alpha2 {
                include!("google.cloud.domains.v1alpha2.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.domains.v1beta1.rs");
            }
        }
        pub mod edgecontainer {
            pub mod v1 {
                include!("google.cloud.edgecontainer.v1.rs");
            }
        }
        pub mod edgenetwork {
            pub mod v1 {
                include!("google.cloud.edgenetwork.v1.rs");
            }
        }
        pub mod enterpriseknowledgegraph {
            pub mod v1 {
                include!("google.cloud.enterpriseknowledgegraph.v1.rs");
            }
        }
        pub mod essentialcontacts {
            pub mod v1 {
                include!("google.cloud.essentialcontacts.v1.rs");
            }
        }
        pub mod eventarc {
            pub mod publishing {
                pub mod v1 {
                    include!("google.cloud.eventarc.publishing.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.eventarc.v1.rs");
            }
        }
        pub mod filestore {
            pub mod v1 {
                include!("google.cloud.filestore.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.filestore.v1beta1.rs");
            }
        }
        pub mod functions {
            pub mod v1 {
                include!("google.cloud.functions.v1.rs");
            }
            pub mod v2 {
                include!("google.cloud.functions.v2.rs");
            }
            pub mod v2alpha {
                include!("google.cloud.functions.v2alpha.rs");
            }
            pub mod v2beta {
                include!("google.cloud.functions.v2beta.rs");
            }
        }
        pub mod gdchardwaremanagement {
            pub mod v1alpha {
                include!("google.cloud.gdchardwaremanagement.v1alpha.rs");
            }
        }
        pub mod gkebackup {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.gkebackup.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.gkebackup.v1.rs");
            }
        }
        pub mod gkeconnect {
            pub mod gateway {
                pub mod v1 {
                    include!("google.cloud.gkeconnect.gateway.v1.rs");
                }
                pub mod v1alpha1 {
                    include!("google.cloud.gkeconnect.gateway.v1alpha1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.gkeconnect.gateway.v1beta1.rs");
                }
            }
        }
        pub mod gkehub {
            pub mod cloudauditlogging {
                pub mod v1alpha {
                    include!("google.cloud.gkehub.cloudauditlogging.v1alpha.rs");
                }
            }
            pub mod configmanagement {
                pub mod v1 {
                    include!("google.cloud.gkehub.configmanagement.v1.rs");
                }
                pub mod v1alpha {
                    include!("google.cloud.gkehub.configmanagement.v1alpha.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.gkehub.configmanagement.v1beta.rs");
                }
            }
            pub mod metering {
                pub mod v1alpha {
                    include!("google.cloud.gkehub.metering.v1alpha.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.gkehub.metering.v1beta.rs");
                }
            }
            pub mod multiclusteringress {
                pub mod v1 {
                    include!("google.cloud.gkehub.multiclusteringress.v1.rs");
                }
                pub mod v1alpha {
                    include!("google.cloud.gkehub.multiclusteringress.v1alpha.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.gkehub.multiclusteringress.v1beta.rs");
                }
            }
            pub mod servicemesh {
                pub mod v1alpha {
                    include!("google.cloud.gkehub.servicemesh.v1alpha.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.gkehub.servicemesh.v1beta.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.gkehub.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.gkehub.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.gkehub.v1beta.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.gkehub.v1beta1.rs");
            }
        }
        pub mod gkemulticloud {
            pub mod v1 {
                include!("google.cloud.gkemulticloud.v1.rs");
            }
        }
        pub mod gsuiteaddons {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.gsuiteaddons.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.gsuiteaddons.v1.rs");
            }
        }
        pub mod healthcare {
            pub mod logging {
                include!("google.cloud.healthcare.logging.rs");
            }
        }
        pub mod iap {
            pub mod v1 {
                include!("google.cloud.iap.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.iap.v1beta1.rs");
            }
        }
        pub mod identitytoolkit {
            pub mod logging {
                include!("google.cloud.identitytoolkit.logging.rs");
            }
            pub mod v2 {
                include!("google.cloud.identitytoolkit.v2.rs");
            }
        }
        pub mod ids {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.ids.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.ids.v1.rs");
            }
        }
        pub mod integrations {
            pub mod v1alpha {
                include!("google.cloud.integrations.v1alpha.rs");
            }
        }
        pub mod iot {
            pub mod v1 {
                include!("google.cloud.iot.v1.rs");
            }
        }
        pub mod kms {
            pub mod inventory {
                pub mod v1 {
                    include!("google.cloud.kms.inventory.v1.rs");
                }
            }
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.kms.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.kms.v1.rs");
            }
        }
        pub mod language {
            pub mod v1 {
                include!("google.cloud.language.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.language.v1beta1.rs");
            }
            pub mod v1beta2 {
                include!("google.cloud.language.v1beta2.rs");
            }
            pub mod v2 {
                include!("google.cloud.language.v2.rs");
            }
        }
        pub mod lifesciences {
            pub mod v2beta {
                include!("google.cloud.lifesciences.v2beta.rs");
            }
        }
        pub mod location {
            include!("google.cloud.location.rs");
        }
        pub mod managedidentities {
            pub mod v1 {
                include!("google.cloud.managedidentities.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.managedidentities.v1beta1.rs");
            }
        }
        pub mod managedkafka {
            pub mod v1 {
                include!("google.cloud.managedkafka.v1.rs");
            }
        }
        pub mod mediatranslation {
            pub mod v1alpha1 {
                include!("google.cloud.mediatranslation.v1alpha1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.mediatranslation.v1beta1.rs");
            }
        }
        pub mod memcache {
            pub mod v1 {
                include!("google.cloud.memcache.v1.rs");
            }
            pub mod v1beta2 {
                include!("google.cloud.memcache.v1beta2.rs");
            }
        }
        pub mod metastore {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.metastore.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.metastore.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.metastore.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.metastore.v1beta.rs");
            }
        }
        pub mod migrationcenter {
            pub mod v1 {
                include!("google.cloud.migrationcenter.v1.rs");
            }
        }
        pub mod netapp {
            pub mod v1 {
                include!("google.cloud.netapp.v1.rs");
            }
        }
        pub mod networkanalyzer {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.networkanalyzer.logging.v1.rs");
                }
            }
        }
        pub mod networkconnectivity {
            pub mod v1 {
                include!("google.cloud.networkconnectivity.v1.rs");
            }
            pub mod v1alpha1 {
                include!("google.cloud.networkconnectivity.v1alpha1.rs");
            }
        }
        pub mod networkmanagement {
            pub mod v1 {
                include!("google.cloud.networkmanagement.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.networkmanagement.v1beta1.rs");
            }
        }
        pub mod networksecurity {
            pub mod v1 {
                include!("google.cloud.networksecurity.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.networksecurity.v1beta1.rs");
            }
        }
        pub mod networkservices {
            pub mod v1 {
                include!("google.cloud.networkservices.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.networkservices.v1beta1.rs");
            }
        }
        pub mod notebooks {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.notebooks.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.notebooks.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.notebooks.v1beta1.rs");
            }
            pub mod v2 {
                include!("google.cloud.notebooks.v2.rs");
            }
        }
        pub mod optimization {
            pub mod v1 {
                include!("google.cloud.optimization.v1.rs");
            }
        }
        pub mod orchestration {
            pub mod airflow {
                pub mod service {
                    pub mod v1 {
                        include!("google.cloud.orchestration.airflow.service.v1.rs");
                    }
                    pub mod v1beta1 {
                        include!("google.cloud.orchestration.airflow.service.v1beta1.rs");
                    }
                }
            }
        }
        pub mod orgpolicy {
            pub mod v1 {
                include!("google.cloud.orgpolicy.v1.rs");
            }
            pub mod v2 {
                include!("google.cloud.orgpolicy.v2.rs");
            }
        }
        pub mod osconfig {
            pub mod agentendpoint {
                pub mod v1 {
                    include!("google.cloud.osconfig.agentendpoint.v1.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.osconfig.agentendpoint.v1beta.rs");
                }
            }
            pub mod logging {
                include!("google.cloud.osconfig.logging.rs");
            }
            pub mod v1 {
                include!("google.cloud.osconfig.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.osconfig.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.osconfig.v1beta.rs");
            }
        }
        pub mod oslogin {
            pub mod common {
                include!("google.cloud.oslogin.common.rs");
            }
            pub mod v1 {
                include!("google.cloud.oslogin.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.oslogin.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.oslogin.v1beta.rs");
            }
        }
        pub mod parallelstore {
            pub mod v1beta {
                include!("google.cloud.parallelstore.v1beta.rs");
            }
        }
        pub mod paymentgateway {
            pub mod issuerswitch {
                pub mod accountmanager {
                    pub mod v1 {
                        include!("google.cloud.paymentgateway.issuerswitch.accountmanager.v1.rs");
                    }
                }
                pub mod v1 {
                    include!("google.cloud.paymentgateway.issuerswitch.v1.rs");
                }
            }
        }
        pub mod phishingprotection {
            pub mod v1beta1 {
                include!("google.cloud.phishingprotection.v1beta1.rs");
            }
        }
        pub mod policysimulator {
            pub mod v1 {
                include!("google.cloud.policysimulator.v1.rs");
            }
        }
        pub mod policytroubleshooter {
            pub mod iam {
                pub mod v3 {
                    include!("google.cloud.policytroubleshooter.iam.v3.rs");
                }
                pub mod v3beta {
                    include!("google.cloud.policytroubleshooter.iam.v3beta.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.policytroubleshooter.v1.rs");
            }
        }
        pub mod privatecatalog {
            pub mod v1beta1 {
                include!("google.cloud.privatecatalog.v1beta1.rs");
            }
        }
        pub mod privilegedaccessmanager {
            pub mod v1 {
                include!("google.cloud.privilegedaccessmanager.v1.rs");
            }
        }
        pub mod pubsublite {
            pub mod v1 {
                include!("google.cloud.pubsublite.v1.rs");
            }
        }
        pub mod rapidmigrationassessment {
            pub mod v1 {
                include!("google.cloud.rapidmigrationassessment.v1.rs");
            }
        }
        pub mod recaptchaenterprise {
            pub mod v1 {
                include!("google.cloud.recaptchaenterprise.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.recaptchaenterprise.v1beta1.rs");
            }
        }
        pub mod recommendationengine {
            pub mod v1beta1 {
                include!("google.cloud.recommendationengine.v1beta1.rs");
            }
        }
        pub mod recommender {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.recommender.logging.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.recommender.logging.v1beta1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.recommender.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.recommender.v1beta1.rs");
            }
        }
        pub mod redis {
            pub mod cluster {
                pub mod v1 {
                    include!("google.cloud.redis.cluster.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.redis.cluster.v1beta1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.redis.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.redis.v1beta1.rs");
            }
        }
        pub mod resourcemanager {
            pub mod v2 {
                include!("google.cloud.resourcemanager.v2.rs");
            }
            pub mod v3 {
                include!("google.cloud.resourcemanager.v3.rs");
            }
        }
        pub mod resourcesettings {
            pub mod v1 {
                include!("google.cloud.resourcesettings.v1.rs");
            }
        }
        pub mod retail {
            pub mod logging {
                include!("google.cloud.retail.logging.rs");
            }
            pub mod v2 {
                include!("google.cloud.retail.v2.rs");
            }
            pub mod v2alpha {
                include!("google.cloud.retail.v2alpha.rs");
            }
            pub mod v2beta {
                include!("google.cloud.retail.v2beta.rs");
            }
        }
        pub mod run {
            pub mod v2 {
                include!("google.cloud.run.v2.rs");
            }
        }
        pub mod runtimeconfig {
            pub mod v1beta1 {
                include!("google.cloud.runtimeconfig.v1beta1.rs");
            }
        }
        pub mod saasaccelerator {
            pub mod management {
                pub mod logs {
                    pub mod v1 {
                        include!("google.cloud.saasaccelerator.management.logs.v1.rs");
                    }
                }
            }
        }
        pub mod scheduler {
            pub mod v1 {
                include!("google.cloud.scheduler.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.scheduler.v1beta1.rs");
            }
        }
        pub mod secretmanager {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.secretmanager.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.secretmanager.v1.rs");
            }
            pub mod v1beta2 {
                include!("google.cloud.secretmanager.v1beta2.rs");
            }
        }
        pub mod secrets {
            pub mod v1beta1 {
                include!("google.cloud.secrets.v1beta1.rs");
            }
        }
        pub mod securesourcemanager {
            pub mod v1 {
                include!("google.cloud.securesourcemanager.v1.rs");
            }
        }
        pub mod security {
            pub mod privateca {
                pub mod v1 {
                    include!("google.cloud.security.privateca.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.security.privateca.v1beta1.rs");
                }
            }
            pub mod publicca {
                pub mod v1 {
                    include!("google.cloud.security.publicca.v1.rs");
                }
                pub mod v1beta1 {
                    include!("google.cloud.security.publicca.v1beta1.rs");
                }
            }
        }
        pub mod securitycenter {
            pub mod settings {
                pub mod v1beta1 {
                    include!("google.cloud.securitycenter.settings.v1beta1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.securitycenter.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.securitycenter.v1beta1.rs");
            }
            pub mod v1p1beta1 {
                include!("google.cloud.securitycenter.v1p1beta1.rs");
            }
            pub mod v2 {
                include!("google.cloud.securitycenter.v2.rs");
            }
        }
        pub mod securitycentermanagement {
            pub mod v1 {
                include!("google.cloud.securitycentermanagement.v1.rs");
            }
        }
        pub mod securityposture {
            pub mod v1 {
                include!("google.cloud.securityposture.v1.rs");
            }
        }
        pub mod sensitiveaction {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.sensitiveaction.logging.v1.rs");
                }
            }
        }
        pub mod servicedirectory {
            pub mod v1 {
                include!("google.cloud.servicedirectory.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.servicedirectory.v1beta1.rs");
            }
        }
        pub mod servicehealth {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.servicehealth.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.servicehealth.v1.rs");
            }
        }
        pub mod shell {
            pub mod v1 {
                include!("google.cloud.shell.v1.rs");
            }
        }
        pub mod speech {
            pub mod v1 {
                include!("google.cloud.speech.v1.rs");
            }
            pub mod v1p1beta1 {
                include!("google.cloud.speech.v1p1beta1.rs");
            }
            pub mod v2 {
                include!("google.cloud.speech.v2.rs");
            }
        }
        pub mod sql {
            pub mod v1 {
                include!("google.cloud.sql.v1.rs");
            }
            pub mod v1beta4 {
                include!("google.cloud.sql.v1beta4.rs");
            }
        }
        pub mod storageinsights {
            pub mod v1 {
                include!("google.cloud.storageinsights.v1.rs");
            }
        }
        pub mod stream {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.stream.logging.v1.rs");
                }
            }
        }
        pub mod support {
            pub mod v2 {
                include!("google.cloud.support.v2.rs");
            }
        }
        pub mod talent {
            pub mod v4 {
                include!("google.cloud.talent.v4.rs");
            }
            pub mod v4beta1 {
                include!("google.cloud.talent.v4beta1.rs");
            }
        }
        pub mod tasks {
            pub mod v2 {
                include!("google.cloud.tasks.v2.rs");
            }
            pub mod v2beta2 {
                include!("google.cloud.tasks.v2beta2.rs");
            }
            pub mod v2beta3 {
                include!("google.cloud.tasks.v2beta3.rs");
            }
        }
        pub mod telcoautomation {
            pub mod v1 {
                include!("google.cloud.telcoautomation.v1.rs");
            }
            pub mod v1alpha1 {
                include!("google.cloud.telcoautomation.v1alpha1.rs");
            }
        }
        pub mod texttospeech {
            pub mod v1 {
                include!("google.cloud.texttospeech.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.texttospeech.v1beta1.rs");
            }
        }
        pub mod timeseriesinsights {
            pub mod v1 {
                include!("google.cloud.timeseriesinsights.v1.rs");
            }
        }
        pub mod tpu {
            pub mod v1 {
                include!("google.cloud.tpu.v1.rs");
            }
            pub mod v2 {
                include!("google.cloud.tpu.v2.rs");
            }
            pub mod v2alpha1 {
                include!("google.cloud.tpu.v2alpha1.rs");
            }
        }
        pub mod translation {
            pub mod v3 {
                include!("google.cloud.translation.v3.rs");
            }
            pub mod v3beta1 {
                include!("google.cloud.translation.v3beta1.rs");
            }
        }
        pub mod video {
            pub mod livestream {
                pub mod logging {
                    pub mod v1 {
                        include!("google.cloud.video.livestream.logging.v1.rs");
                    }
                }
                pub mod v1 {
                    include!("google.cloud.video.livestream.v1.rs");
                }
            }
            pub mod stitcher {
                pub mod v1 {
                    include!("google.cloud.video.stitcher.v1.rs");
                }
            }
            pub mod transcoder {
                pub mod v1 {
                    include!("google.cloud.video.transcoder.v1.rs");
                }
            }
        }
        pub mod videointelligence {
            pub mod v1 {
                include!("google.cloud.videointelligence.v1.rs");
            }
            pub mod v1beta2 {
                include!("google.cloud.videointelligence.v1beta2.rs");
            }
            pub mod v1p1beta1 {
                include!("google.cloud.videointelligence.v1p1beta1.rs");
            }
            pub mod v1p2beta1 {
                include!("google.cloud.videointelligence.v1p2beta1.rs");
            }
            pub mod v1p3beta1 {
                include!("google.cloud.videointelligence.v1p3beta1.rs");
            }
        }
        pub mod vision {
            pub mod v1 {
                include!("google.cloud.vision.v1.rs");
            }
            pub mod v1p1beta1 {
                include!("google.cloud.vision.v1p1beta1.rs");
            }
            pub mod v1p2beta1 {
                include!("google.cloud.vision.v1p2beta1.rs");
            }
            pub mod v1p3beta1 {
                include!("google.cloud.vision.v1p3beta1.rs");
            }
            pub mod v1p4beta1 {
                include!("google.cloud.vision.v1p4beta1.rs");
            }
        }
        pub mod visionai {
            pub mod v1 {
                include!("google.cloud.visionai.v1.rs");
            }
            pub mod v1alpha1 {
                include!("google.cloud.visionai.v1alpha1.rs");
            }
        }
        pub mod vmmigration {
            pub mod v1 {
                include!("google.cloud.vmmigration.v1.rs");
            }
        }
        pub mod vmwareengine {
            pub mod v1 {
                include!("google.cloud.vmwareengine.v1.rs");
            }
        }
        pub mod vpcaccess {
            pub mod v1 {
                include!("google.cloud.vpcaccess.v1.rs");
            }
        }
        pub mod webrisk {
            pub mod v1 {
                include!("google.cloud.webrisk.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.cloud.webrisk.v1beta1.rs");
            }
        }
        pub mod websecurityscanner {
            pub mod v1 {
                include!("google.cloud.websecurityscanner.v1.rs");
            }
            pub mod v1alpha {
                include!("google.cloud.websecurityscanner.v1alpha.rs");
            }
            pub mod v1beta {
                include!("google.cloud.websecurityscanner.v1beta.rs");
            }
        }
        pub mod workflows {
            pub mod executions {
                pub mod v1 {
                    include!("google.cloud.workflows.executions.v1.rs");
                }
                pub mod v1beta {
                    include!("google.cloud.workflows.executions.v1beta.rs");
                }
            }
            pub mod r#type {
                include!("google.cloud.workflows.r#type.rs");
            }
            pub mod v1 {
                include!("google.cloud.workflows.v1.rs");
            }
            pub mod v1beta {
                include!("google.cloud.workflows.v1beta.rs");
            }
        }
        pub mod workstations {
            pub mod logging {
                pub mod v1 {
                    include!("google.cloud.workstations.logging.v1.rs");
                }
            }
            pub mod v1 {
                include!("google.cloud.workstations.v1.rs");
            }
            pub mod v1beta {
                include!("google.cloud.workstations.v1beta.rs");
            }
        }
    }
    pub mod compute {
        pub mod logging {
            pub mod dr {
                pub mod v1 {
                    include!("google.compute.logging.dr.v1.rs");
                }
            }
            pub mod gdnsusage {
                pub mod v1 {
                    include!("google.compute.logging.gdnsusage.v1.rs");
                }
            }
        }
    }
    pub mod container {
        pub mod v1 {
            include!("google.container.v1.rs");
        }
        pub mod v1alpha1 {
            include!("google.container.v1alpha1.rs");
        }
        pub mod v1beta1 {
            include!("google.container.v1beta1.rs");
        }
    }
    pub mod dataflow {
        pub mod v1beta3 {
            include!("google.dataflow.v1beta3.rs");
        }
    }
    pub mod datastore {
        pub mod admin {
            pub mod v1 {
                include!("google.datastore.admin.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.datastore.admin.v1beta1.rs");
            }
        }
        pub mod v1 {
            include!("google.datastore.v1.rs");
        }
        pub mod v1beta3 {
            include!("google.datastore.v1beta3.rs");
        }
    }
    pub mod devtools {
        pub mod artifactregistry {
            pub mod v1 {
                include!("google.devtools.artifactregistry.v1.rs");
            }
            pub mod v1beta2 {
                include!("google.devtools.artifactregistry.v1beta2.rs");
            }
        }
        pub mod build {
            pub mod v1 {
                include!("google.devtools.build.v1.rs");
            }
        }
        pub mod cloudbuild {
            pub mod v1 {
                include!("google.devtools.cloudbuild.v1.rs");
            }
            pub mod v2 {
                include!("google.devtools.cloudbuild.v2.rs");
            }
        }
        pub mod clouddebugger {
            pub mod v2 {
                include!("google.devtools.clouddebugger.v2.rs");
            }
        }
        pub mod clouderrorreporting {
            pub mod v1beta1 {
                include!("google.devtools.clouderrorreporting.v1beta1.rs");
            }
        }
        pub mod cloudprofiler {
            pub mod v2 {
                include!("google.devtools.cloudprofiler.v2.rs");
            }
        }
        pub mod cloudtrace {
            pub mod v1 {
                include!("google.devtools.cloudtrace.v1.rs");
            }
            pub mod v2 {
                include!("google.devtools.cloudtrace.v2.rs");
            }
        }
        pub mod containeranalysis {
            pub mod v1 {
                include!("google.devtools.containeranalysis.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.devtools.containeranalysis.v1beta1.rs");
            }
        }
        pub mod remoteworkers {
            pub mod v1test2 {
                include!("google.devtools.remoteworkers.v1test2.rs");
            }
        }
        pub mod resultstore {
            pub mod v2 {
                include!("google.devtools.resultstore.v2.rs");
            }
        }
        pub mod source {
            pub mod v1 {
                include!("google.devtools.source.v1.rs");
            }
        }
        pub mod sourcerepo {
            pub mod v1 {
                include!("google.devtools.sourcerepo.v1.rs");
            }
        }
        pub mod testing {
            pub mod v1 {
                include!("google.devtools.testing.v1.rs");
            }
        }
    }
    pub mod example {
        pub mod library {
            pub mod v1 {
                include!("google.example.library.v1.rs");
            }
        }
    }
    pub mod firebase {
        pub mod fcm {
            pub mod connection {
                pub mod v1alpha1 {
                    include!("google.firebase.fcm.connection.v1alpha1.rs");
                }
            }
        }
    }
    pub mod firestore {
        pub mod admin {
            pub mod v1 {
                include!("google.firestore.admin.v1.rs");
            }
            pub mod v1beta1 {
                include!("google.firestore.admin.v1beta1.rs");
            }
            pub mod v1beta2 {
                include!("google.firestore.admin.v1beta2.rs");
            }
        }
        pub mod bundle {
            include!("google.firestore.bundle.rs");
        }
        pub mod v1 {
            include!("google.firestore.v1.rs");
        }
        pub mod v1beta1 {
            include!("google.firestore.v1beta1.rs");
        }
    }
    pub mod gapic {
        pub mod metadata {
            include!("google.gapic.metadata.rs");
        }
    }
    pub mod genomics {
        pub mod v1 {
            include!("google.genomics.v1.rs");
        }
        pub mod v1alpha2 {
            include!("google.genomics.v1alpha2.rs");
        }
    }
    pub mod geo {
        pub mod r#type {
            include!("google.geo.r#type.rs");
        }
    }
    pub mod home {
        pub mod enterprise {
            pub mod sdm {
                pub mod v1 {
                    include!("google.home.enterprise.sdm.v1.rs");
                }
            }
        }
        pub mod graph {
            pub mod v1 {
                include!("google.home.graph.v1.rs");
            }
        }
    }
    pub mod iam {
        pub mod admin {
            pub mod v1 {
                include!("google.iam.admin.v1.rs");
            }
        }
        pub mod credentials {
            pub mod v1 {
                include!("google.iam.credentials.v1.rs");
            }
        }
        pub mod v1 {
            include!("google.iam.v1.rs");
            pub mod logging {
                include!("google.iam.v1.logging.rs");
            }
        }
        pub mod v1beta {
            include!("google.iam.v1beta.rs");
        }
        pub mod v2 {
            include!("google.iam.v2.rs");
        }
        pub mod v2beta {
            include!("google.iam.v2beta.rs");
        }
    }
    pub mod identity {
        pub mod accesscontextmanager {
            pub mod r#type {
                include!("google.identity.accesscontextmanager.r#type.rs");
            }
            pub mod v1 {
                include!("google.identity.accesscontextmanager.v1.rs");
            }
        }
    }
    pub mod logging {
        pub mod r#type {
            include!("google.logging.r#type.rs");
        }
        pub mod v2 {
            include!("google.logging.v2.rs");
        }
    }
    pub mod longrunning {
        include!("google.longrunning.rs");
    }
    pub mod maps {
        pub mod addressvalidation {
            pub mod v1 {
                include!("google.maps.addressvalidation.v1.rs");
            }
        }
        pub mod aerialview {
            pub mod v1 {
                include!("google.maps.aerialview.v1.rs");
            }
        }
        pub mod mapsplatformdatasets {
            pub mod v1 {
                include!("google.maps.mapsplatformdatasets.v1.rs");
            }
        }
        pub mod mobilitybilling {
            pub mod logs {
                pub mod v1 {
                    include!("google.maps.mobilitybilling.logs.v1.rs");
                }
            }
        }
        pub mod places {
            pub mod v1 {
                include!("google.maps.places.v1.rs");
            }
        }
        pub mod playablelocations {
            pub mod v3 {
                include!("google.maps.playablelocations.v3.rs");
                pub mod sample {
                    include!("google.maps.playablelocations.v3.sample.rs");
                }
            }
        }
        pub mod regionlookup {
            pub mod v1alpha {
                include!("google.maps.regionlookup.v1alpha.rs");
            }
        }
        pub mod roads {
            pub mod v1op {
                include!("google.maps.roads.v1op.rs");
            }
        }
        pub mod routeoptimization {
            pub mod v1 {
                include!("google.maps.routeoptimization.v1.rs");
            }
        }
        pub mod routes {
            pub mod v1 {
                include!("google.maps.routes.v1.rs");
            }
            pub mod v1alpha {
                include!("google.maps.routes.v1alpha.rs");
            }
        }
        pub mod routing {
            pub mod v2 {
                include!("google.maps.routing.v2.rs");
            }
        }
        pub mod solar {
            pub mod v1 {
                include!("google.maps.solar.v1.rs");
            }
        }
        pub mod unity {
            include!("google.maps.unity.rs");
        }
    }
    pub mod marketingplatform {
        pub mod admin {
            pub mod v1alpha {
                include!("google.marketingplatform.admin.v1alpha.rs");
            }
        }
    }
    pub mod monitoring {
        pub mod dashboard {
            pub mod v1 {
                include!("google.monitoring.dashboard.v1.rs");
            }
        }
        pub mod metricsscope {
            pub mod v1 {
                include!("google.monitoring.metricsscope.v1.rs");
            }
        }
        pub mod v3 {
            include!("google.monitoring.v3.rs");
        }
    }
    pub mod networking {
        pub mod trafficdirector {
            pub mod r#type {
                include!("google.networking.trafficdirector.r#type.rs");
            }
        }
    }
    pub mod partner {
        pub mod aistreams {
            pub mod v1alpha1 {
                include!("google.partner.aistreams.v1alpha1.rs");
            }
        }
    }
    pub mod privacy {
        pub mod dlp {
            pub mod v2 {
                include!("google.privacy.dlp.v2.rs");
            }
        }
    }
    pub mod pubsub {
        pub mod v1 {
            include!("google.pubsub.v1.rs");
        }
        pub mod v1beta2 {
            include!("google.pubsub.v1beta2.rs");
        }
    }
    pub mod r#type {
        include!("google.r#type.rs");
    }
    pub mod rpc {
        include!("google.rpc.rs");
        pub mod context {
            include!("google.rpc.context.rs");
        }
    }
    pub mod search {
        pub mod partnerdataingestion {
            pub mod logging {
                pub mod v1 {
                    include!("google.search.partnerdataingestion.logging.v1.rs");
                }
            }
        }
    }
    pub mod shopping {
        pub mod css {
            pub mod v1 {
                include!("google.shopping.css.v1.rs");
            }
        }
        pub mod merchant {
            pub mod accounts {
                pub mod v1beta {
                    include!("google.shopping.merchant.accounts.v1beta.rs");
                }
            }
            pub mod conversions {
                pub mod v1beta {
                    include!("google.shopping.merchant.conversions.v1beta.rs");
                }
            }
            pub mod datasources {
                pub mod v1beta {
                    include!("google.shopping.merchant.datasources.v1beta.rs");
                }
            }
            pub mod inventories {
                pub mod v1beta {
                    include!("google.shopping.merchant.inventories.v1beta.rs");
                }
            }
            pub mod lfp {
                pub mod v1beta {
                    include!("google.shopping.merchant.lfp.v1beta.rs");
                }
            }
            pub mod notifications {
                pub mod v1beta {
                    include!("google.shopping.merchant.notifications.v1beta.rs");
                }
            }
            pub mod products {
                pub mod v1beta {
                    include!("google.shopping.merchant.products.v1beta.rs");
                }
            }
            pub mod promotions {
                pub mod v1beta {
                    include!("google.shopping.merchant.promotions.v1beta.rs");
                }
            }
            pub mod quota {
                pub mod v1beta {
                    include!("google.shopping.merchant.quota.v1beta.rs");
                }
            }
            pub mod reports {
                pub mod v1beta {
                    include!("google.shopping.merchant.reports.v1beta.rs");
                }
            }
        }
        pub mod r#type {
            include!("google.shopping.r#type.rs");
        }
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    include!("google.spanner.admin.database.v1.rs");
                }
            }
            pub mod instance {
                pub mod v1 {
                    include!("google.spanner.admin.instance.v1.rs");
                }
            }
        }
        pub mod executor {
            pub mod v1 {
                include!("google.spanner.executor.v1.rs");
            }
        }
        pub mod v1 {
            include!("google.spanner.v1.rs");
        }
    }
    pub mod storage {
        pub mod control {
            pub mod v2 {
                include!("google.storage.control.v2.rs");
            }
        }
        pub mod v1 {
            include!("google.storage.v1.rs");
        }
        pub mod v2 {
            include!("google.storage.v2.rs");
        }
    }
    pub mod storagetransfer {
        pub mod logging {
            include!("google.storagetransfer.logging.rs");
        }
        pub mod v1 {
            include!("google.storagetransfer.v1.rs");
        }
    }
    pub mod streetview {
        pub mod publish {
            pub mod v1 {
                include!("google.streetview.publish.v1.rs");
            }
        }
    }
    pub mod watcher {
        pub mod v1 {
            include!("google.watcher.v1.rs");
        }
    }
}
pub mod grafeas {
    pub mod v1 {
        include!("grafeas.v1.rs");
    }
    pub mod v1beta1 {
        include!("grafeas.v1beta1.rs");
        pub mod attestation {
            include!("grafeas.v1beta1.attestation.rs");
        }
        pub mod build {
            include!("grafeas.v1beta1.build.rs");
        }
        pub mod deployment {
            include!("grafeas.v1beta1.deployment.rs");
        }
        pub mod discovery {
            include!("grafeas.v1beta1.discovery.rs");
        }
        pub mod image {
            include!("grafeas.v1beta1.image.rs");
        }
        pub mod package {
            include!("grafeas.v1beta1.package.rs");
        }
        pub mod provenance {
            include!("grafeas.v1beta1.provenance.rs");
        }
        pub mod source {
            include!("grafeas.v1beta1.source.rs");
        }
        pub mod vulnerability {
            include!("grafeas.v1beta1.vulnerability.rs");
        }
    }
}
pub mod maps {
    pub mod fleetengine {
        pub mod delivery {
            pub mod v1 {
                include!("maps.fleetengine.delivery.v1.rs");
            }
        }
        pub mod v1 {
            include!("maps.fleetengine.v1.rs");
        }
    }
}
