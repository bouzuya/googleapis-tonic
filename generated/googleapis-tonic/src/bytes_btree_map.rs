#[cfg(any(
    feature = "google-actions-sdk-v2",
    feature = "google-actions-sdk-v2-conversation",
    feature = "google-actions-sdk-v2-interactionmodel",
    feature = "google-actions-sdk-v2-interactionmodel-prompt",
    feature = "google-actions-sdk-v2-interactionmodel-type",
    feature = "google-actions-type",
    feature = "google-ads-admanager-v1",
    feature = "google-ads-admob-v1",
    feature = "google-ads-googleads-v15-common",
    feature = "google-ads-googleads-v15-enums",
    feature = "google-ads-googleads-v15-errors",
    feature = "google-ads-googleads-v15-resources",
    feature = "google-ads-googleads-v15-services",
    feature = "google-ads-googleads-v16-common",
    feature = "google-ads-googleads-v16-enums",
    feature = "google-ads-googleads-v16-errors",
    feature = "google-ads-googleads-v16-resources",
    feature = "google-ads-googleads-v16-services",
    feature = "google-ads-googleads-v17-common",
    feature = "google-ads-googleads-v17-enums",
    feature = "google-ads-googleads-v17-errors",
    feature = "google-ads-googleads-v17-resources",
    feature = "google-ads-googleads-v17-services",
    feature = "google-ads-searchads360-v0-common",
    feature = "google-ads-searchads360-v0-enums",
    feature = "google-ads-searchads360-v0-errors",
    feature = "google-ads-searchads360-v0-resources",
    feature = "google-ads-searchads360-v0-services",
    feature = "google-ai-generativelanguage-v1",
    feature = "google-ai-generativelanguage-v1beta",
    feature = "google-ai-generativelanguage-v1beta2",
    feature = "google-ai-generativelanguage-v1beta3",
    feature = "google-analytics-admin-v1alpha",
    feature = "google-analytics-admin-v1beta",
    feature = "google-analytics-data-v1alpha",
    feature = "google-analytics-data-v1beta",
    feature = "google-api",
    feature = "google-api-apikeys-v2",
    feature = "google-api-cloudquotas-v1",
    feature = "google-api-expr-conformance-v1alpha1",
    feature = "google-api-expr-v1alpha1",
    feature = "google-api-expr-v1beta1",
    feature = "google-api-servicecontrol-v1",
    feature = "google-api-servicecontrol-v2",
    feature = "google-api-servicemanagement-v1",
    feature = "google-api-serviceusage-v1",
    feature = "google-api-serviceusage-v1beta1",
    feature = "google-appengine-legacy",
    feature = "google-appengine-logging-v1",
    feature = "google-appengine-v1",
    feature = "google-appengine-v1beta",
    feature = "google-apps-alertcenter-v1beta1",
    feature = "google-apps-card-v1",
    feature = "google-apps-drive-activity-v2",
    feature = "google-apps-drive-labels-v2",
    feature = "google-apps-drive-labels-v2beta",
    feature = "google-apps-events-subscriptions-v1",
    feature = "google-apps-meet-v2",
    feature = "google-apps-meet-v2beta",
    feature = "google-apps-script-type",
    feature = "google-apps-script-type-calendar",
    feature = "google-apps-script-type-docs",
    feature = "google-apps-script-type-drive",
    feature = "google-apps-script-type-gmail",
    feature = "google-apps-script-type-sheets",
    feature = "google-apps-script-type-slides",
    feature = "google-area120-tables-v1alpha1",
    feature = "google-assistant-embedded-v1alpha1",
    feature = "google-assistant-embedded-v1alpha2",
    feature = "google-bigtable-admin-v2",
    feature = "google-bigtable-v2",
    feature = "google-bytestream",
    feature = "google-chat-logging-v1",
    feature = "google-chat-v1",
    feature = "google-chromeos-moblab-v1beta1",
    feature = "google-chromeos-uidetection-v1",
    feature = "google-cloud",
    feature = "google-cloud-abuseevent-logging-v1",
    feature = "google-cloud-accessapproval-v1",
    feature = "google-cloud-advisorynotifications-v1",
    feature = "google-cloud-aiplatform-logging",
    feature = "google-cloud-aiplatform-v1",
    feature = "google-cloud-aiplatform-v1-schema-predict-instance",
    feature = "google-cloud-aiplatform-v1-schema-predict-params",
    feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
    feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
    feature = "google-cloud-aiplatform-v1beta1",
    feature = "google-cloud-aiplatform-v1beta1-schema",
    feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
    feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
    feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
    feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
    feature = "google-cloud-alloydb-connectors-v1",
    feature = "google-cloud-alloydb-connectors-v1alpha",
    feature = "google-cloud-alloydb-connectors-v1beta",
    feature = "google-cloud-alloydb-v1",
    feature = "google-cloud-alloydb-v1alpha",
    feature = "google-cloud-alloydb-v1beta",
    feature = "google-cloud-apigateway-v1",
    feature = "google-cloud-apigeeconnect-v1",
    feature = "google-cloud-apigeeregistry-v1",
    feature = "google-cloud-apihub-v1",
    feature = "google-cloud-apphub-v1",
    feature = "google-cloud-asset-v1",
    feature = "google-cloud-asset-v1p1beta1",
    feature = "google-cloud-asset-v1p2beta1",
    feature = "google-cloud-asset-v1p5beta1",
    feature = "google-cloud-asset-v1p7beta1",
    feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
    feature = "google-cloud-assuredworkloads-v1",
    feature = "google-cloud-assuredworkloads-v1beta1",
    feature = "google-cloud-audit",
    feature = "google-cloud-automl-v1",
    feature = "google-cloud-automl-v1beta1",
    feature = "google-cloud-backupdr-logging-v1",
    feature = "google-cloud-backupdr-v1",
    feature = "google-cloud-baremetalsolution-v2",
    feature = "google-cloud-batch-v1",
    feature = "google-cloud-batch-v1alpha",
    feature = "google-cloud-beyondcorp-appconnections-v1",
    feature = "google-cloud-beyondcorp-appconnectors-v1",
    feature = "google-cloud-beyondcorp-appgateways-v1",
    feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
    feature = "google-cloud-beyondcorp-clientgateways-v1",
    feature = "google-cloud-bigquery-analyticshub-v1",
    feature = "google-cloud-bigquery-biglake-v1",
    feature = "google-cloud-bigquery-biglake-v1alpha1",
    feature = "google-cloud-bigquery-connection-v1",
    feature = "google-cloud-bigquery-connection-v1beta1",
    feature = "google-cloud-bigquery-dataexchange-v1beta1",
    feature = "google-cloud-bigquery-datapolicies-v1",
    feature = "google-cloud-bigquery-datapolicies-v1beta1",
    feature = "google-cloud-bigquery-datatransfer-v1",
    feature = "google-cloud-bigquery-logging-v1",
    feature = "google-cloud-bigquery-migration-v2",
    feature = "google-cloud-bigquery-migration-v2alpha",
    feature = "google-cloud-bigquery-reservation-v1",
    feature = "google-cloud-bigquery-storage-v1",
    feature = "google-cloud-bigquery-storage-v1alpha",
    feature = "google-cloud-bigquery-storage-v1beta1",
    feature = "google-cloud-bigquery-storage-v1beta2",
    feature = "google-cloud-bigquery-v2",
    feature = "google-cloud-billing-budgets-v1",
    feature = "google-cloud-billing-budgets-v1beta1",
    feature = "google-cloud-billing-v1",
    feature = "google-cloud-binaryauthorization-v1",
    feature = "google-cloud-binaryauthorization-v1beta1",
    feature = "google-cloud-blockchainnodeengine-v1",
    feature = "google-cloud-certificatemanager-logging-v1",
    feature = "google-cloud-certificatemanager-v1",
    feature = "google-cloud-channel-v1",
    feature = "google-cloud-cloudcontrolspartner-v1",
    feature = "google-cloud-cloudcontrolspartner-v1beta",
    feature = "google-cloud-clouddms-logging-v1",
    feature = "google-cloud-clouddms-v1",
    feature = "google-cloud-cloudsetup-logging-v1",
    feature = "google-cloud-commerce-consumer-procurement-v1",
    feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
    feature = "google-cloud-common",
    feature = "google-cloud-compute-v1",
    feature = "google-cloud-compute-v1small",
    feature = "google-cloud-confidentialcomputing-v1",
    feature = "google-cloud-confidentialcomputing-v1alpha1",
    feature = "google-cloud-config-v1",
    feature = "google-cloud-connectors-v1",
    feature = "google-cloud-contactcenterinsights-v1",
    feature = "google-cloud-contentwarehouse-v1",
    feature = "google-cloud-datacatalog-lineage-v1",
    feature = "google-cloud-datacatalog-v1",
    feature = "google-cloud-datacatalog-v1beta1",
    feature = "google-cloud-dataform-logging-v1",
    feature = "google-cloud-dataform-v1alpha2",
    feature = "google-cloud-dataform-v1beta1",
    feature = "google-cloud-datafusion-v1",
    feature = "google-cloud-datafusion-v1beta1",
    feature = "google-cloud-datalabeling-v1beta1",
    feature = "google-cloud-datapipelines-logging-v1",
    feature = "google-cloud-dataplex-v1",
    feature = "google-cloud-dataproc-logging",
    feature = "google-cloud-dataproc-v1",
    feature = "google-cloud-dataqna-v1alpha",
    feature = "google-cloud-datastream-logging-v1",
    feature = "google-cloud-datastream-v1",
    feature = "google-cloud-datastream-v1alpha1",
    feature = "google-cloud-deploy-v1",
    feature = "google-cloud-developerconnect-v1",
    feature = "google-cloud-dialogflow-cx-v3",
    feature = "google-cloud-dialogflow-cx-v3beta1",
    feature = "google-cloud-dialogflow-v2",
    feature = "google-cloud-dialogflow-v2beta1",
    feature = "google-cloud-discoveryengine-logging",
    feature = "google-cloud-discoveryengine-v1",
    feature = "google-cloud-discoveryengine-v1alpha",
    feature = "google-cloud-discoveryengine-v1beta",
    feature = "google-cloud-documentai-v1",
    feature = "google-cloud-documentai-v1beta1",
    feature = "google-cloud-documentai-v1beta2",
    feature = "google-cloud-documentai-v1beta3",
    feature = "google-cloud-domains-v1",
    feature = "google-cloud-domains-v1alpha2",
    feature = "google-cloud-domains-v1beta1",
    feature = "google-cloud-edgecontainer-v1",
    feature = "google-cloud-edgenetwork-v1",
    feature = "google-cloud-enterpriseknowledgegraph-v1",
    feature = "google-cloud-essentialcontacts-v1",
    feature = "google-cloud-eventarc-publishing-v1",
    feature = "google-cloud-eventarc-v1",
    feature = "google-cloud-filestore-v1",
    feature = "google-cloud-filestore-v1beta1",
    feature = "google-cloud-functions-v1",
    feature = "google-cloud-functions-v2",
    feature = "google-cloud-functions-v2alpha",
    feature = "google-cloud-functions-v2beta",
    feature = "google-cloud-gdchardwaremanagement-v1alpha",
    feature = "google-cloud-gkebackup-logging-v1",
    feature = "google-cloud-gkebackup-v1",
    feature = "google-cloud-gkeconnect-gateway-v1",
    feature = "google-cloud-gkeconnect-gateway-v1beta1",
    feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
    feature = "google-cloud-gkehub-configmanagement-v1",
    feature = "google-cloud-gkehub-configmanagement-v1alpha",
    feature = "google-cloud-gkehub-configmanagement-v1beta",
    feature = "google-cloud-gkehub-metering-v1alpha",
    feature = "google-cloud-gkehub-metering-v1beta",
    feature = "google-cloud-gkehub-multiclusteringress-v1",
    feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
    feature = "google-cloud-gkehub-multiclusteringress-v1beta",
    feature = "google-cloud-gkehub-servicemesh-v1alpha",
    feature = "google-cloud-gkehub-servicemesh-v1beta",
    feature = "google-cloud-gkehub-v1",
    feature = "google-cloud-gkehub-v1alpha",
    feature = "google-cloud-gkehub-v1beta",
    feature = "google-cloud-gkehub-v1beta1",
    feature = "google-cloud-gkemulticloud-v1",
    feature = "google-cloud-gsuiteaddons-logging-v1",
    feature = "google-cloud-gsuiteaddons-v1",
    feature = "google-cloud-healthcare-logging",
    feature = "google-cloud-iap-v1",
    feature = "google-cloud-iap-v1beta1",
    feature = "google-cloud-identitytoolkit-logging",
    feature = "google-cloud-identitytoolkit-v2",
    feature = "google-cloud-ids-logging-v1",
    feature = "google-cloud-ids-v1",
    feature = "google-cloud-integrations-v1alpha",
    feature = "google-cloud-iot-v1",
    feature = "google-cloud-kms-inventory-v1",
    feature = "google-cloud-kms-logging-v1",
    feature = "google-cloud-kms-v1",
    feature = "google-cloud-language-v1",
    feature = "google-cloud-language-v1beta1",
    feature = "google-cloud-language-v1beta2",
    feature = "google-cloud-language-v2",
    feature = "google-cloud-lifesciences-v2beta",
    feature = "google-cloud-location",
    feature = "google-cloud-managedidentities-v1",
    feature = "google-cloud-managedidentities-v1beta1",
    feature = "google-cloud-managedkafka-v1",
    feature = "google-cloud-mediatranslation-v1alpha1",
    feature = "google-cloud-mediatranslation-v1beta1",
    feature = "google-cloud-memcache-v1",
    feature = "google-cloud-memcache-v1beta2",
    feature = "google-cloud-metastore-logging-v1",
    feature = "google-cloud-metastore-v1",
    feature = "google-cloud-metastore-v1alpha",
    feature = "google-cloud-metastore-v1beta",
    feature = "google-cloud-migrationcenter-v1",
    feature = "google-cloud-netapp-v1",
    feature = "google-cloud-networkanalyzer-logging-v1",
    feature = "google-cloud-networkconnectivity-v1",
    feature = "google-cloud-networkconnectivity-v1alpha1",
    feature = "google-cloud-networkmanagement-v1",
    feature = "google-cloud-networkmanagement-v1beta1",
    feature = "google-cloud-networksecurity-v1",
    feature = "google-cloud-networksecurity-v1beta1",
    feature = "google-cloud-networkservices-v1",
    feature = "google-cloud-networkservices-v1beta1",
    feature = "google-cloud-notebooks-logging-v1",
    feature = "google-cloud-notebooks-v1",
    feature = "google-cloud-notebooks-v1beta1",
    feature = "google-cloud-notebooks-v2",
    feature = "google-cloud-optimization-v1",
    feature = "google-cloud-orchestration-airflow-service-v1",
    feature = "google-cloud-orchestration-airflow-service-v1beta1",
    feature = "google-cloud-orgpolicy-v1",
    feature = "google-cloud-orgpolicy-v2",
    feature = "google-cloud-osconfig-agentendpoint-v1",
    feature = "google-cloud-osconfig-agentendpoint-v1beta",
    feature = "google-cloud-osconfig-logging",
    feature = "google-cloud-osconfig-v1",
    feature = "google-cloud-osconfig-v1alpha",
    feature = "google-cloud-osconfig-v1beta",
    feature = "google-cloud-oslogin-common",
    feature = "google-cloud-oslogin-v1",
    feature = "google-cloud-oslogin-v1alpha",
    feature = "google-cloud-oslogin-v1beta",
    feature = "google-cloud-parallelstore-v1beta",
    feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
    feature = "google-cloud-paymentgateway-issuerswitch-v1",
    feature = "google-cloud-phishingprotection-v1beta1",
    feature = "google-cloud-policysimulator-v1",
    feature = "google-cloud-policytroubleshooter-iam-v3",
    feature = "google-cloud-policytroubleshooter-iam-v3beta",
    feature = "google-cloud-policytroubleshooter-v1",
    feature = "google-cloud-privatecatalog-v1beta1",
    feature = "google-cloud-privilegedaccessmanager-v1",
    feature = "google-cloud-pubsublite-v1",
    feature = "google-cloud-rapidmigrationassessment-v1",
    feature = "google-cloud-recaptchaenterprise-v1",
    feature = "google-cloud-recaptchaenterprise-v1beta1",
    feature = "google-cloud-recommendationengine-v1beta1",
    feature = "google-cloud-recommender-logging-v1",
    feature = "google-cloud-recommender-logging-v1beta1",
    feature = "google-cloud-recommender-v1",
    feature = "google-cloud-recommender-v1beta1",
    feature = "google-cloud-redis-cluster-v1",
    feature = "google-cloud-redis-cluster-v1beta1",
    feature = "google-cloud-redis-v1",
    feature = "google-cloud-redis-v1beta1",
    feature = "google-cloud-resourcemanager-v2",
    feature = "google-cloud-resourcemanager-v3",
    feature = "google-cloud-resourcesettings-v1",
    feature = "google-cloud-retail-logging",
    feature = "google-cloud-retail-v2",
    feature = "google-cloud-retail-v2alpha",
    feature = "google-cloud-retail-v2beta",
    feature = "google-cloud-run-v2",
    feature = "google-cloud-runtimeconfig-v1beta1",
    feature = "google-cloud-saasaccelerator-management-logs-v1",
    feature = "google-cloud-scheduler-v1",
    feature = "google-cloud-scheduler-v1beta1",
    feature = "google-cloud-secretmanager-logging-v1",
    feature = "google-cloud-secretmanager-v1",
    feature = "google-cloud-secretmanager-v1beta2",
    feature = "google-cloud-secrets-v1beta1",
    feature = "google-cloud-securesourcemanager-v1",
    feature = "google-cloud-security-privateca-v1",
    feature = "google-cloud-security-privateca-v1beta1",
    feature = "google-cloud-security-publicca-v1",
    feature = "google-cloud-security-publicca-v1beta1",
    feature = "google-cloud-securitycenter-settings-v1beta1",
    feature = "google-cloud-securitycenter-v1",
    feature = "google-cloud-securitycenter-v1beta1",
    feature = "google-cloud-securitycenter-v1p1beta1",
    feature = "google-cloud-securitycenter-v2",
    feature = "google-cloud-securitycentermanagement-v1",
    feature = "google-cloud-securityposture-v1",
    feature = "google-cloud-sensitiveaction-logging-v1",
    feature = "google-cloud-servicedirectory-v1",
    feature = "google-cloud-servicedirectory-v1beta1",
    feature = "google-cloud-servicehealth-logging-v1",
    feature = "google-cloud-servicehealth-v1",
    feature = "google-cloud-shell-v1",
    feature = "google-cloud-speech-v1",
    feature = "google-cloud-speech-v1p1beta1",
    feature = "google-cloud-speech-v2",
    feature = "google-cloud-sql-v1",
    feature = "google-cloud-sql-v1beta4",
    feature = "google-cloud-storageinsights-v1",
    feature = "google-cloud-stream-logging-v1",
    feature = "google-cloud-support-v2",
    feature = "google-cloud-talent-v4",
    feature = "google-cloud-talent-v4beta1",
    feature = "google-cloud-tasks-v2",
    feature = "google-cloud-tasks-v2beta2",
    feature = "google-cloud-tasks-v2beta3",
    feature = "google-cloud-telcoautomation-v1",
    feature = "google-cloud-telcoautomation-v1alpha1",
    feature = "google-cloud-texttospeech-v1",
    feature = "google-cloud-texttospeech-v1beta1",
    feature = "google-cloud-timeseriesinsights-v1",
    feature = "google-cloud-tpu-v1",
    feature = "google-cloud-tpu-v2",
    feature = "google-cloud-tpu-v2alpha1",
    feature = "google-cloud-translation-v3",
    feature = "google-cloud-translation-v3beta1",
    feature = "google-cloud-video-livestream-logging-v1",
    feature = "google-cloud-video-livestream-v1",
    feature = "google-cloud-video-stitcher-v1",
    feature = "google-cloud-video-transcoder-v1",
    feature = "google-cloud-videointelligence-v1",
    feature = "google-cloud-videointelligence-v1beta2",
    feature = "google-cloud-videointelligence-v1p1beta1",
    feature = "google-cloud-videointelligence-v1p2beta1",
    feature = "google-cloud-videointelligence-v1p3beta1",
    feature = "google-cloud-vision-v1",
    feature = "google-cloud-vision-v1p1beta1",
    feature = "google-cloud-vision-v1p2beta1",
    feature = "google-cloud-vision-v1p3beta1",
    feature = "google-cloud-vision-v1p4beta1",
    feature = "google-cloud-visionai-v1",
    feature = "google-cloud-visionai-v1alpha1",
    feature = "google-cloud-vmmigration-v1",
    feature = "google-cloud-vmwareengine-v1",
    feature = "google-cloud-vpcaccess-v1",
    feature = "google-cloud-webrisk-v1",
    feature = "google-cloud-webrisk-v1beta1",
    feature = "google-cloud-websecurityscanner-v1",
    feature = "google-cloud-websecurityscanner-v1alpha",
    feature = "google-cloud-websecurityscanner-v1beta",
    feature = "google-cloud-workflows-executions-v1",
    feature = "google-cloud-workflows-executions-v1beta",
    feature = "google-cloud-workflows-type",
    feature = "google-cloud-workflows-v1",
    feature = "google-cloud-workflows-v1beta",
    feature = "google-cloud-workstations-logging-v1",
    feature = "google-cloud-workstations-v1",
    feature = "google-cloud-workstations-v1beta",
    feature = "google-compute-logging-dr-v1",
    feature = "google-compute-logging-gdnsusage-v1",
    feature = "google-container-v1",
    feature = "google-container-v1alpha1",
    feature = "google-container-v1beta1",
    feature = "google-dataflow-v1beta3",
    feature = "google-datastore-admin-v1",
    feature = "google-datastore-admin-v1beta1",
    feature = "google-datastore-v1",
    feature = "google-datastore-v1beta3",
    feature = "google-devtools-artifactregistry-v1",
    feature = "google-devtools-artifactregistry-v1beta2",
    feature = "google-devtools-build-v1",
    feature = "google-devtools-cloudbuild-v1",
    feature = "google-devtools-cloudbuild-v2",
    feature = "google-devtools-clouddebugger-v2",
    feature = "google-devtools-clouderrorreporting-v1beta1",
    feature = "google-devtools-cloudprofiler-v2",
    feature = "google-devtools-cloudtrace-v1",
    feature = "google-devtools-cloudtrace-v2",
    feature = "google-devtools-containeranalysis-v1",
    feature = "google-devtools-containeranalysis-v1beta1",
    feature = "google-devtools-remoteworkers-v1test2",
    feature = "google-devtools-resultstore-v2",
    feature = "google-devtools-source-v1",
    feature = "google-devtools-sourcerepo-v1",
    feature = "google-devtools-testing-v1",
    feature = "google-example-library-v1",
    feature = "google-firebase-fcm-connection-v1alpha1",
    feature = "google-firestore-admin-v1",
    feature = "google-firestore-admin-v1beta1",
    feature = "google-firestore-admin-v1beta2",
    feature = "google-firestore-bundle",
    feature = "google-firestore-v1",
    feature = "google-firestore-v1beta1",
    feature = "google-gapic-metadata",
    feature = "google-genomics-v1",
    feature = "google-genomics-v1alpha2",
    feature = "google-geo-type",
    feature = "google-home-enterprise-sdm-v1",
    feature = "google-home-graph-v1",
    feature = "google-iam-admin-v1",
    feature = "google-iam-credentials-v1",
    feature = "google-iam-v1",
    feature = "google-iam-v1-logging",
    feature = "google-iam-v1beta",
    feature = "google-iam-v2",
    feature = "google-iam-v2beta",
    feature = "google-identity-accesscontextmanager-type",
    feature = "google-identity-accesscontextmanager-v1",
    feature = "google-logging-type",
    feature = "google-logging-v2",
    feature = "google-longrunning",
    feature = "google-maps-addressvalidation-v1",
    feature = "google-maps-aerialview-v1",
    feature = "google-maps-areainsights-v1",
    feature = "google-maps-mapsplatformdatasets-v1",
    feature = "google-maps-mobilitybilling-logs-v1",
    feature = "google-maps-places-v1",
    feature = "google-maps-playablelocations-v3",
    feature = "google-maps-playablelocations-v3-sample",
    feature = "google-maps-regionlookup-v1alpha",
    feature = "google-maps-roads-v1op",
    feature = "google-maps-routeoptimization-v1",
    feature = "google-maps-routes-v1",
    feature = "google-maps-routes-v1alpha",
    feature = "google-maps-routing-v2",
    feature = "google-maps-solar-v1",
    feature = "google-maps-unity",
    feature = "google-marketingplatform-admin-v1alpha",
    feature = "google-monitoring-dashboard-v1",
    feature = "google-monitoring-metricsscope-v1",
    feature = "google-monitoring-v3",
    feature = "google-networking-trafficdirector-type",
    feature = "google-partner-aistreams-v1alpha1",
    feature = "google-privacy-dlp-v2",
    feature = "google-pubsub-v1",
    feature = "google-pubsub-v1beta2",
    feature = "google-rpc",
    feature = "google-rpc-context",
    feature = "google-search-partnerdataingestion-logging-v1",
    feature = "google-shopping-css-v1",
    feature = "google-shopping-merchant-accounts-v1beta",
    feature = "google-shopping-merchant-conversions-v1beta",
    feature = "google-shopping-merchant-datasources-v1beta",
    feature = "google-shopping-merchant-inventories-v1beta",
    feature = "google-shopping-merchant-lfp-v1beta",
    feature = "google-shopping-merchant-notifications-v1beta",
    feature = "google-shopping-merchant-products-v1beta",
    feature = "google-shopping-merchant-promotions-v1beta",
    feature = "google-shopping-merchant-quota-v1beta",
    feature = "google-shopping-merchant-reports-v1beta",
    feature = "google-shopping-type",
    feature = "google-spanner-admin-database-v1",
    feature = "google-spanner-admin-instance-v1",
    feature = "google-spanner-executor-v1",
    feature = "google-spanner-v1",
    feature = "google-storage-control-v2",
    feature = "google-storage-v1",
    feature = "google-storage-v2",
    feature = "google-storagetransfer-logging",
    feature = "google-storagetransfer-v1",
    feature = "google-streetview-publish-v1",
    feature = "google-type",
    feature = "google-watcher-v1",
))]
pub mod google {
    #[cfg(any(
        feature = "google-apps-alertcenter-v1beta1",
        feature = "google-apps-card-v1",
        feature = "google-apps-drive-activity-v2",
        feature = "google-apps-drive-labels-v2",
        feature = "google-apps-drive-labels-v2beta",
        feature = "google-apps-events-subscriptions-v1",
        feature = "google-apps-meet-v2",
        feature = "google-apps-meet-v2beta",
        feature = "google-apps-script-type",
        feature = "google-apps-script-type-calendar",
        feature = "google-apps-script-type-docs",
        feature = "google-apps-script-type-drive",
        feature = "google-apps-script-type-gmail",
        feature = "google-apps-script-type-sheets",
        feature = "google-apps-script-type-slides",
    ))]
    pub mod apps {
        #[cfg(any(
            feature = "google-apps-card-v1",
        ))]
        pub mod card {
            #[cfg(any(
                feature = "google-apps-card-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-apps-card-v1",
                ))]
                include!("bytes_btree_map/google.apps.card.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-apps-script-type",
            feature = "google-apps-script-type-calendar",
            feature = "google-apps-script-type-docs",
            feature = "google-apps-script-type-drive",
            feature = "google-apps-script-type-gmail",
            feature = "google-apps-script-type-sheets",
            feature = "google-apps-script-type-slides",
        ))]
        pub mod script {
            #[cfg(any(
                feature = "google-apps-script-type",
                feature = "google-apps-script-type-calendar",
                feature = "google-apps-script-type-docs",
                feature = "google-apps-script-type-drive",
                feature = "google-apps-script-type-gmail",
                feature = "google-apps-script-type-sheets",
                feature = "google-apps-script-type-slides",
            ))]
            pub mod r#type {
                #[cfg(any(
                    feature = "google-apps-script-type",
                ))]
                include!("bytes_btree_map/google.apps.script.r#type.rs");
                #[cfg(any(
                    feature = "google-apps-script-type-calendar",
                ))]
                pub mod calendar {
                    #[cfg(any(
                        feature = "google-apps-script-type-calendar",
                    ))]
                    include!("bytes_btree_map/google.apps.script.r#type.calendar.rs");
                }
                #[cfg(any(
                    feature = "google-apps-script-type-gmail",
                ))]
                pub mod gmail {
                    #[cfg(any(
                        feature = "google-apps-script-type-gmail",
                    ))]
                    include!("bytes_btree_map/google.apps.script.r#type.gmail.rs");
                }
                #[cfg(any(
                    feature = "google-apps-script-type-docs",
                ))]
                pub mod docs {
                    #[cfg(any(
                        feature = "google-apps-script-type-docs",
                    ))]
                    include!("bytes_btree_map/google.apps.script.r#type.docs.rs");
                }
                #[cfg(any(
                    feature = "google-apps-script-type-sheets",
                ))]
                pub mod sheets {
                    #[cfg(any(
                        feature = "google-apps-script-type-sheets",
                    ))]
                    include!("bytes_btree_map/google.apps.script.r#type.sheets.rs");
                }
                #[cfg(any(
                    feature = "google-apps-script-type-drive",
                ))]
                pub mod drive {
                    #[cfg(any(
                        feature = "google-apps-script-type-drive",
                    ))]
                    include!("bytes_btree_map/google.apps.script.r#type.drive.rs");
                }
                #[cfg(any(
                    feature = "google-apps-script-type-slides",
                ))]
                pub mod slides {
                    #[cfg(any(
                        feature = "google-apps-script-type-slides",
                    ))]
                    include!("bytes_btree_map/google.apps.script.r#type.slides.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-apps-events-subscriptions-v1",
        ))]
        pub mod events {
            #[cfg(any(
                feature = "google-apps-events-subscriptions-v1",
            ))]
            pub mod subscriptions {
                #[cfg(any(
                    feature = "google-apps-events-subscriptions-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-apps-events-subscriptions-v1",
                    ))]
                    include!("bytes_btree_map/google.apps.events.subscriptions.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-apps-alertcenter-v1beta1",
        ))]
        pub mod alertcenter {
            #[cfg(any(
                feature = "google-apps-alertcenter-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-apps-alertcenter-v1beta1",
                ))]
                include!("bytes_btree_map/google.apps.alertcenter.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-apps-meet-v2",
            feature = "google-apps-meet-v2beta",
        ))]
        pub mod meet {
            #[cfg(any(
                feature = "google-apps-meet-v2beta",
            ))]
            pub mod v2beta {
                #[cfg(any(
                    feature = "google-apps-meet-v2beta",
                ))]
                include!("bytes_btree_map/google.apps.meet.v2beta.rs");
            }
            #[cfg(any(
                feature = "google-apps-meet-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-apps-meet-v2",
                ))]
                include!("bytes_btree_map/google.apps.meet.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-apps-drive-activity-v2",
            feature = "google-apps-drive-labels-v2",
            feature = "google-apps-drive-labels-v2beta",
        ))]
        pub mod drive {
            #[cfg(any(
                feature = "google-apps-drive-labels-v2",
                feature = "google-apps-drive-labels-v2beta",
            ))]
            pub mod labels {
                #[cfg(any(
                    feature = "google-apps-drive-labels-v2",
                ))]
                pub mod v2 {
                    #[cfg(any(
                        feature = "google-apps-drive-labels-v2",
                    ))]
                    include!("bytes_btree_map/google.apps.drive.labels.v2.rs");
                }
                #[cfg(any(
                    feature = "google-apps-drive-labels-v2beta",
                ))]
                pub mod v2beta {
                    #[cfg(any(
                        feature = "google-apps-drive-labels-v2beta",
                    ))]
                    include!("bytes_btree_map/google.apps.drive.labels.v2beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-apps-drive-activity-v2",
            ))]
            pub mod activity {
                #[cfg(any(
                    feature = "google-apps-drive-activity-v2",
                ))]
                pub mod v2 {
                    #[cfg(any(
                        feature = "google-apps-drive-activity-v2",
                    ))]
                    include!("bytes_btree_map/google.apps.drive.activity.v2.rs");
                }
            }
        }
    }
    #[cfg(any(
        feature = "google-cloud",
        feature = "google-cloud-abuseevent-logging-v1",
        feature = "google-cloud-accessapproval-v1",
        feature = "google-cloud-advisorynotifications-v1",
        feature = "google-cloud-aiplatform-logging",
        feature = "google-cloud-aiplatform-v1",
        feature = "google-cloud-aiplatform-v1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
        feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
        feature = "google-cloud-aiplatform-v1beta1",
        feature = "google-cloud-aiplatform-v1beta1-schema",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
        feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
        feature = "google-cloud-alloydb-connectors-v1",
        feature = "google-cloud-alloydb-connectors-v1alpha",
        feature = "google-cloud-alloydb-connectors-v1beta",
        feature = "google-cloud-alloydb-v1",
        feature = "google-cloud-alloydb-v1alpha",
        feature = "google-cloud-alloydb-v1beta",
        feature = "google-cloud-apigateway-v1",
        feature = "google-cloud-apigeeconnect-v1",
        feature = "google-cloud-apigeeregistry-v1",
        feature = "google-cloud-apihub-v1",
        feature = "google-cloud-apphub-v1",
        feature = "google-cloud-asset-v1",
        feature = "google-cloud-asset-v1p1beta1",
        feature = "google-cloud-asset-v1p2beta1",
        feature = "google-cloud-asset-v1p5beta1",
        feature = "google-cloud-asset-v1p7beta1",
        feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
        feature = "google-cloud-assuredworkloads-v1",
        feature = "google-cloud-assuredworkloads-v1beta1",
        feature = "google-cloud-audit",
        feature = "google-cloud-automl-v1",
        feature = "google-cloud-automl-v1beta1",
        feature = "google-cloud-backupdr-logging-v1",
        feature = "google-cloud-backupdr-v1",
        feature = "google-cloud-baremetalsolution-v2",
        feature = "google-cloud-batch-v1",
        feature = "google-cloud-batch-v1alpha",
        feature = "google-cloud-beyondcorp-appconnections-v1",
        feature = "google-cloud-beyondcorp-appconnectors-v1",
        feature = "google-cloud-beyondcorp-appgateways-v1",
        feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
        feature = "google-cloud-beyondcorp-clientgateways-v1",
        feature = "google-cloud-bigquery-analyticshub-v1",
        feature = "google-cloud-bigquery-biglake-v1",
        feature = "google-cloud-bigquery-biglake-v1alpha1",
        feature = "google-cloud-bigquery-connection-v1",
        feature = "google-cloud-bigquery-connection-v1beta1",
        feature = "google-cloud-bigquery-dataexchange-v1beta1",
        feature = "google-cloud-bigquery-datapolicies-v1",
        feature = "google-cloud-bigquery-datapolicies-v1beta1",
        feature = "google-cloud-bigquery-datatransfer-v1",
        feature = "google-cloud-bigquery-logging-v1",
        feature = "google-cloud-bigquery-migration-v2",
        feature = "google-cloud-bigquery-migration-v2alpha",
        feature = "google-cloud-bigquery-reservation-v1",
        feature = "google-cloud-bigquery-storage-v1",
        feature = "google-cloud-bigquery-storage-v1alpha",
        feature = "google-cloud-bigquery-storage-v1beta1",
        feature = "google-cloud-bigquery-storage-v1beta2",
        feature = "google-cloud-bigquery-v2",
        feature = "google-cloud-billing-budgets-v1",
        feature = "google-cloud-billing-budgets-v1beta1",
        feature = "google-cloud-billing-v1",
        feature = "google-cloud-binaryauthorization-v1",
        feature = "google-cloud-binaryauthorization-v1beta1",
        feature = "google-cloud-blockchainnodeengine-v1",
        feature = "google-cloud-certificatemanager-logging-v1",
        feature = "google-cloud-certificatemanager-v1",
        feature = "google-cloud-channel-v1",
        feature = "google-cloud-cloudcontrolspartner-v1",
        feature = "google-cloud-cloudcontrolspartner-v1beta",
        feature = "google-cloud-clouddms-logging-v1",
        feature = "google-cloud-clouddms-v1",
        feature = "google-cloud-cloudsetup-logging-v1",
        feature = "google-cloud-commerce-consumer-procurement-v1",
        feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
        feature = "google-cloud-common",
        feature = "google-cloud-compute-v1",
        feature = "google-cloud-compute-v1small",
        feature = "google-cloud-confidentialcomputing-v1",
        feature = "google-cloud-confidentialcomputing-v1alpha1",
        feature = "google-cloud-config-v1",
        feature = "google-cloud-connectors-v1",
        feature = "google-cloud-contactcenterinsights-v1",
        feature = "google-cloud-contentwarehouse-v1",
        feature = "google-cloud-datacatalog-lineage-v1",
        feature = "google-cloud-datacatalog-v1",
        feature = "google-cloud-datacatalog-v1beta1",
        feature = "google-cloud-dataform-logging-v1",
        feature = "google-cloud-dataform-v1alpha2",
        feature = "google-cloud-dataform-v1beta1",
        feature = "google-cloud-datafusion-v1",
        feature = "google-cloud-datafusion-v1beta1",
        feature = "google-cloud-datalabeling-v1beta1",
        feature = "google-cloud-datapipelines-logging-v1",
        feature = "google-cloud-dataplex-v1",
        feature = "google-cloud-dataproc-logging",
        feature = "google-cloud-dataproc-v1",
        feature = "google-cloud-dataqna-v1alpha",
        feature = "google-cloud-datastream-logging-v1",
        feature = "google-cloud-datastream-v1",
        feature = "google-cloud-datastream-v1alpha1",
        feature = "google-cloud-deploy-v1",
        feature = "google-cloud-developerconnect-v1",
        feature = "google-cloud-dialogflow-cx-v3",
        feature = "google-cloud-dialogflow-cx-v3beta1",
        feature = "google-cloud-dialogflow-v2",
        feature = "google-cloud-dialogflow-v2beta1",
        feature = "google-cloud-discoveryengine-logging",
        feature = "google-cloud-discoveryengine-v1",
        feature = "google-cloud-discoveryengine-v1alpha",
        feature = "google-cloud-discoveryengine-v1beta",
        feature = "google-cloud-documentai-v1",
        feature = "google-cloud-documentai-v1beta1",
        feature = "google-cloud-documentai-v1beta2",
        feature = "google-cloud-documentai-v1beta3",
        feature = "google-cloud-domains-v1",
        feature = "google-cloud-domains-v1alpha2",
        feature = "google-cloud-domains-v1beta1",
        feature = "google-cloud-edgecontainer-v1",
        feature = "google-cloud-edgenetwork-v1",
        feature = "google-cloud-enterpriseknowledgegraph-v1",
        feature = "google-cloud-essentialcontacts-v1",
        feature = "google-cloud-eventarc-publishing-v1",
        feature = "google-cloud-eventarc-v1",
        feature = "google-cloud-filestore-v1",
        feature = "google-cloud-filestore-v1beta1",
        feature = "google-cloud-functions-v1",
        feature = "google-cloud-functions-v2",
        feature = "google-cloud-functions-v2alpha",
        feature = "google-cloud-functions-v2beta",
        feature = "google-cloud-gdchardwaremanagement-v1alpha",
        feature = "google-cloud-gkebackup-logging-v1",
        feature = "google-cloud-gkebackup-v1",
        feature = "google-cloud-gkeconnect-gateway-v1",
        feature = "google-cloud-gkeconnect-gateway-v1beta1",
        feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
        feature = "google-cloud-gkehub-configmanagement-v1",
        feature = "google-cloud-gkehub-configmanagement-v1alpha",
        feature = "google-cloud-gkehub-configmanagement-v1beta",
        feature = "google-cloud-gkehub-metering-v1alpha",
        feature = "google-cloud-gkehub-metering-v1beta",
        feature = "google-cloud-gkehub-multiclusteringress-v1",
        feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
        feature = "google-cloud-gkehub-multiclusteringress-v1beta",
        feature = "google-cloud-gkehub-servicemesh-v1alpha",
        feature = "google-cloud-gkehub-servicemesh-v1beta",
        feature = "google-cloud-gkehub-v1",
        feature = "google-cloud-gkehub-v1alpha",
        feature = "google-cloud-gkehub-v1beta",
        feature = "google-cloud-gkehub-v1beta1",
        feature = "google-cloud-gkemulticloud-v1",
        feature = "google-cloud-gsuiteaddons-logging-v1",
        feature = "google-cloud-gsuiteaddons-v1",
        feature = "google-cloud-healthcare-logging",
        feature = "google-cloud-iap-v1",
        feature = "google-cloud-iap-v1beta1",
        feature = "google-cloud-identitytoolkit-logging",
        feature = "google-cloud-identitytoolkit-v2",
        feature = "google-cloud-ids-logging-v1",
        feature = "google-cloud-ids-v1",
        feature = "google-cloud-integrations-v1alpha",
        feature = "google-cloud-iot-v1",
        feature = "google-cloud-kms-inventory-v1",
        feature = "google-cloud-kms-logging-v1",
        feature = "google-cloud-kms-v1",
        feature = "google-cloud-language-v1",
        feature = "google-cloud-language-v1beta1",
        feature = "google-cloud-language-v1beta2",
        feature = "google-cloud-language-v2",
        feature = "google-cloud-lifesciences-v2beta",
        feature = "google-cloud-location",
        feature = "google-cloud-managedidentities-v1",
        feature = "google-cloud-managedidentities-v1beta1",
        feature = "google-cloud-managedkafka-v1",
        feature = "google-cloud-mediatranslation-v1alpha1",
        feature = "google-cloud-mediatranslation-v1beta1",
        feature = "google-cloud-memcache-v1",
        feature = "google-cloud-memcache-v1beta2",
        feature = "google-cloud-metastore-logging-v1",
        feature = "google-cloud-metastore-v1",
        feature = "google-cloud-metastore-v1alpha",
        feature = "google-cloud-metastore-v1beta",
        feature = "google-cloud-migrationcenter-v1",
        feature = "google-cloud-netapp-v1",
        feature = "google-cloud-networkanalyzer-logging-v1",
        feature = "google-cloud-networkconnectivity-v1",
        feature = "google-cloud-networkconnectivity-v1alpha1",
        feature = "google-cloud-networkmanagement-v1",
        feature = "google-cloud-networkmanagement-v1beta1",
        feature = "google-cloud-networksecurity-v1",
        feature = "google-cloud-networksecurity-v1beta1",
        feature = "google-cloud-networkservices-v1",
        feature = "google-cloud-networkservices-v1beta1",
        feature = "google-cloud-notebooks-logging-v1",
        feature = "google-cloud-notebooks-v1",
        feature = "google-cloud-notebooks-v1beta1",
        feature = "google-cloud-notebooks-v2",
        feature = "google-cloud-optimization-v1",
        feature = "google-cloud-orchestration-airflow-service-v1",
        feature = "google-cloud-orchestration-airflow-service-v1beta1",
        feature = "google-cloud-orgpolicy-v1",
        feature = "google-cloud-orgpolicy-v2",
        feature = "google-cloud-osconfig-agentendpoint-v1",
        feature = "google-cloud-osconfig-agentendpoint-v1beta",
        feature = "google-cloud-osconfig-logging",
        feature = "google-cloud-osconfig-v1",
        feature = "google-cloud-osconfig-v1alpha",
        feature = "google-cloud-osconfig-v1beta",
        feature = "google-cloud-oslogin-common",
        feature = "google-cloud-oslogin-v1",
        feature = "google-cloud-oslogin-v1alpha",
        feature = "google-cloud-oslogin-v1beta",
        feature = "google-cloud-parallelstore-v1beta",
        feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
        feature = "google-cloud-paymentgateway-issuerswitch-v1",
        feature = "google-cloud-phishingprotection-v1beta1",
        feature = "google-cloud-policysimulator-v1",
        feature = "google-cloud-policytroubleshooter-iam-v3",
        feature = "google-cloud-policytroubleshooter-iam-v3beta",
        feature = "google-cloud-policytroubleshooter-v1",
        feature = "google-cloud-privatecatalog-v1beta1",
        feature = "google-cloud-privilegedaccessmanager-v1",
        feature = "google-cloud-pubsublite-v1",
        feature = "google-cloud-rapidmigrationassessment-v1",
        feature = "google-cloud-recaptchaenterprise-v1",
        feature = "google-cloud-recaptchaenterprise-v1beta1",
        feature = "google-cloud-recommendationengine-v1beta1",
        feature = "google-cloud-recommender-logging-v1",
        feature = "google-cloud-recommender-logging-v1beta1",
        feature = "google-cloud-recommender-v1",
        feature = "google-cloud-recommender-v1beta1",
        feature = "google-cloud-redis-cluster-v1",
        feature = "google-cloud-redis-cluster-v1beta1",
        feature = "google-cloud-redis-v1",
        feature = "google-cloud-redis-v1beta1",
        feature = "google-cloud-resourcemanager-v2",
        feature = "google-cloud-resourcemanager-v3",
        feature = "google-cloud-resourcesettings-v1",
        feature = "google-cloud-retail-logging",
        feature = "google-cloud-retail-v2",
        feature = "google-cloud-retail-v2alpha",
        feature = "google-cloud-retail-v2beta",
        feature = "google-cloud-run-v2",
        feature = "google-cloud-runtimeconfig-v1beta1",
        feature = "google-cloud-saasaccelerator-management-logs-v1",
        feature = "google-cloud-scheduler-v1",
        feature = "google-cloud-scheduler-v1beta1",
        feature = "google-cloud-secretmanager-logging-v1",
        feature = "google-cloud-secretmanager-v1",
        feature = "google-cloud-secretmanager-v1beta2",
        feature = "google-cloud-secrets-v1beta1",
        feature = "google-cloud-securesourcemanager-v1",
        feature = "google-cloud-security-privateca-v1",
        feature = "google-cloud-security-privateca-v1beta1",
        feature = "google-cloud-security-publicca-v1",
        feature = "google-cloud-security-publicca-v1beta1",
        feature = "google-cloud-securitycenter-settings-v1beta1",
        feature = "google-cloud-securitycenter-v1",
        feature = "google-cloud-securitycenter-v1beta1",
        feature = "google-cloud-securitycenter-v1p1beta1",
        feature = "google-cloud-securitycenter-v2",
        feature = "google-cloud-securitycentermanagement-v1",
        feature = "google-cloud-securityposture-v1",
        feature = "google-cloud-sensitiveaction-logging-v1",
        feature = "google-cloud-servicedirectory-v1",
        feature = "google-cloud-servicedirectory-v1beta1",
        feature = "google-cloud-servicehealth-logging-v1",
        feature = "google-cloud-servicehealth-v1",
        feature = "google-cloud-shell-v1",
        feature = "google-cloud-speech-v1",
        feature = "google-cloud-speech-v1p1beta1",
        feature = "google-cloud-speech-v2",
        feature = "google-cloud-sql-v1",
        feature = "google-cloud-sql-v1beta4",
        feature = "google-cloud-storageinsights-v1",
        feature = "google-cloud-stream-logging-v1",
        feature = "google-cloud-support-v2",
        feature = "google-cloud-talent-v4",
        feature = "google-cloud-talent-v4beta1",
        feature = "google-cloud-tasks-v2",
        feature = "google-cloud-tasks-v2beta2",
        feature = "google-cloud-tasks-v2beta3",
        feature = "google-cloud-telcoautomation-v1",
        feature = "google-cloud-telcoautomation-v1alpha1",
        feature = "google-cloud-texttospeech-v1",
        feature = "google-cloud-texttospeech-v1beta1",
        feature = "google-cloud-timeseriesinsights-v1",
        feature = "google-cloud-tpu-v1",
        feature = "google-cloud-tpu-v2",
        feature = "google-cloud-tpu-v2alpha1",
        feature = "google-cloud-translation-v3",
        feature = "google-cloud-translation-v3beta1",
        feature = "google-cloud-video-livestream-logging-v1",
        feature = "google-cloud-video-livestream-v1",
        feature = "google-cloud-video-stitcher-v1",
        feature = "google-cloud-video-transcoder-v1",
        feature = "google-cloud-videointelligence-v1",
        feature = "google-cloud-videointelligence-v1beta2",
        feature = "google-cloud-videointelligence-v1p1beta1",
        feature = "google-cloud-videointelligence-v1p2beta1",
        feature = "google-cloud-videointelligence-v1p3beta1",
        feature = "google-cloud-vision-v1",
        feature = "google-cloud-vision-v1p1beta1",
        feature = "google-cloud-vision-v1p2beta1",
        feature = "google-cloud-vision-v1p3beta1",
        feature = "google-cloud-vision-v1p4beta1",
        feature = "google-cloud-visionai-v1",
        feature = "google-cloud-visionai-v1alpha1",
        feature = "google-cloud-vmmigration-v1",
        feature = "google-cloud-vmwareengine-v1",
        feature = "google-cloud-vpcaccess-v1",
        feature = "google-cloud-webrisk-v1",
        feature = "google-cloud-webrisk-v1beta1",
        feature = "google-cloud-websecurityscanner-v1",
        feature = "google-cloud-websecurityscanner-v1alpha",
        feature = "google-cloud-websecurityscanner-v1beta",
        feature = "google-cloud-workflows-executions-v1",
        feature = "google-cloud-workflows-executions-v1beta",
        feature = "google-cloud-workflows-type",
        feature = "google-cloud-workflows-v1",
        feature = "google-cloud-workflows-v1beta",
        feature = "google-cloud-workstations-logging-v1",
        feature = "google-cloud-workstations-v1",
        feature = "google-cloud-workstations-v1beta",
    ))]
    pub mod cloud {
        #[cfg(any(
            feature = "google-cloud",
        ))]
        include!("bytes_btree_map/google.cloud.rs");
        #[cfg(any(
            feature = "google-cloud-privilegedaccessmanager-v1",
        ))]
        pub mod privilegedaccessmanager {
            #[cfg(any(
                feature = "google-cloud-privilegedaccessmanager-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-privilegedaccessmanager-v1",
                ))]
                include!("bytes_btree_map/google.cloud.privilegedaccessmanager.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-alloydb-connectors-v1",
            feature = "google-cloud-alloydb-connectors-v1alpha",
            feature = "google-cloud-alloydb-connectors-v1beta",
            feature = "google-cloud-alloydb-v1",
            feature = "google-cloud-alloydb-v1alpha",
            feature = "google-cloud-alloydb-v1beta",
        ))]
        pub mod alloydb {
            #[cfg(any(
                feature = "google-cloud-alloydb-connectors-v1",
                feature = "google-cloud-alloydb-connectors-v1alpha",
                feature = "google-cloud-alloydb-connectors-v1beta",
            ))]
            pub mod connectors {
                #[cfg(any(
                    feature = "google-cloud-alloydb-connectors-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-alloydb-connectors-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.alloydb.connectors.v1alpha.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-alloydb-connectors-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-alloydb-connectors-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.alloydb.connectors.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-alloydb-connectors-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-alloydb-connectors-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.alloydb.connectors.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-alloydb-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-alloydb-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.alloydb.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-alloydb-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-alloydb-v1",
                ))]
                include!("bytes_btree_map/google.cloud.alloydb.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-alloydb-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-alloydb-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.alloydb.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-eventarc-publishing-v1",
            feature = "google-cloud-eventarc-v1",
        ))]
        pub mod eventarc {
            #[cfg(any(
                feature = "google-cloud-eventarc-publishing-v1",
            ))]
            pub mod publishing {
                #[cfg(any(
                    feature = "google-cloud-eventarc-publishing-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-eventarc-publishing-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.eventarc.publishing.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-eventarc-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-eventarc-v1",
                ))]
                include!("bytes_btree_map/google.cloud.eventarc.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-beyondcorp-appconnections-v1",
            feature = "google-cloud-beyondcorp-appconnectors-v1",
            feature = "google-cloud-beyondcorp-appgateways-v1",
            feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
            feature = "google-cloud-beyondcorp-clientgateways-v1",
        ))]
        pub mod beyondcorp {
            #[cfg(any(
                feature = "google-cloud-beyondcorp-appconnections-v1",
            ))]
            pub mod appconnections {
                #[cfg(any(
                    feature = "google-cloud-beyondcorp-appconnections-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-beyondcorp-appconnections-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.beyondcorp.appconnections.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-beyondcorp-appgateways-v1",
            ))]
            pub mod appgateways {
                #[cfg(any(
                    feature = "google-cloud-beyondcorp-appgateways-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-beyondcorp-appgateways-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.beyondcorp.appgateways.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-beyondcorp-clientgateways-v1",
            ))]
            pub mod clientgateways {
                #[cfg(any(
                    feature = "google-cloud-beyondcorp-clientgateways-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-beyondcorp-clientgateways-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.beyondcorp.clientgateways.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
            ))]
            pub mod clientconnectorservices {
                #[cfg(any(
                    feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-beyondcorp-clientconnectorservices-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.beyondcorp.clientconnectorservices.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-beyondcorp-appconnectors-v1",
            ))]
            pub mod appconnectors {
                #[cfg(any(
                    feature = "google-cloud-beyondcorp-appconnectors-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-beyondcorp-appconnectors-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.beyondcorp.appconnectors.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-functions-v1",
            feature = "google-cloud-functions-v2",
            feature = "google-cloud-functions-v2alpha",
            feature = "google-cloud-functions-v2beta",
        ))]
        pub mod functions {
            #[cfg(any(
                feature = "google-cloud-functions-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-functions-v2",
                ))]
                include!("bytes_btree_map/google.cloud.functions.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-functions-v2beta",
            ))]
            pub mod v2beta {
                #[cfg(any(
                    feature = "google-cloud-functions-v2beta",
                ))]
                include!("bytes_btree_map/google.cloud.functions.v2beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-functions-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-functions-v1",
                ))]
                include!("bytes_btree_map/google.cloud.functions.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-functions-v2alpha",
            ))]
            pub mod v2alpha {
                #[cfg(any(
                    feature = "google-cloud-functions-v2alpha",
                ))]
                include!("bytes_btree_map/google.cloud.functions.v2alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-support-v2",
        ))]
        pub mod support {
            #[cfg(any(
                feature = "google-cloud-support-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-support-v2",
                ))]
                include!("bytes_btree_map/google.cloud.support.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-videointelligence-v1",
            feature = "google-cloud-videointelligence-v1beta2",
            feature = "google-cloud-videointelligence-v1p1beta1",
            feature = "google-cloud-videointelligence-v1p2beta1",
            feature = "google-cloud-videointelligence-v1p3beta1",
        ))]
        pub mod videointelligence {
            #[cfg(any(
                feature = "google-cloud-videointelligence-v1p1beta1",
            ))]
            pub mod v1p1beta1 {
                #[cfg(any(
                    feature = "google-cloud-videointelligence-v1p1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.videointelligence.v1p1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-videointelligence-v1p2beta1",
            ))]
            pub mod v1p2beta1 {
                #[cfg(any(
                    feature = "google-cloud-videointelligence-v1p2beta1",
                ))]
                include!("bytes_btree_map/google.cloud.videointelligence.v1p2beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-videointelligence-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-cloud-videointelligence-v1beta2",
                ))]
                include!("bytes_btree_map/google.cloud.videointelligence.v1beta2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-videointelligence-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-videointelligence-v1",
                ))]
                include!("bytes_btree_map/google.cloud.videointelligence.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-videointelligence-v1p3beta1",
            ))]
            pub mod v1p3beta1 {
                #[cfg(any(
                    feature = "google-cloud-videointelligence-v1p3beta1",
                ))]
                include!("bytes_btree_map/google.cloud.videointelligence.v1p3beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-bigquery-analyticshub-v1",
            feature = "google-cloud-bigquery-biglake-v1",
            feature = "google-cloud-bigquery-biglake-v1alpha1",
            feature = "google-cloud-bigquery-connection-v1",
            feature = "google-cloud-bigquery-connection-v1beta1",
            feature = "google-cloud-bigquery-dataexchange-v1beta1",
            feature = "google-cloud-bigquery-datapolicies-v1",
            feature = "google-cloud-bigquery-datapolicies-v1beta1",
            feature = "google-cloud-bigquery-datatransfer-v1",
            feature = "google-cloud-bigquery-logging-v1",
            feature = "google-cloud-bigquery-migration-v2",
            feature = "google-cloud-bigquery-migration-v2alpha",
            feature = "google-cloud-bigquery-reservation-v1",
            feature = "google-cloud-bigquery-storage-v1",
            feature = "google-cloud-bigquery-storage-v1alpha",
            feature = "google-cloud-bigquery-storage-v1beta1",
            feature = "google-cloud-bigquery-storage-v1beta2",
            feature = "google-cloud-bigquery-v2",
        ))]
        pub mod bigquery {
            #[cfg(any(
                feature = "google-cloud-bigquery-analyticshub-v1",
            ))]
            pub mod analyticshub {
                #[cfg(any(
                    feature = "google-cloud-bigquery-analyticshub-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-analyticshub-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.analyticshub.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-datapolicies-v1",
                feature = "google-cloud-bigquery-datapolicies-v1beta1",
            ))]
            pub mod datapolicies {
                #[cfg(any(
                    feature = "google-cloud-bigquery-datapolicies-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-datapolicies-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.datapolicies.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-datapolicies-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-datapolicies-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.datapolicies.v1beta1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-storage-v1",
                feature = "google-cloud-bigquery-storage-v1alpha",
                feature = "google-cloud-bigquery-storage-v1beta1",
                feature = "google-cloud-bigquery-storage-v1beta2",
            ))]
            pub mod storage {
                #[cfg(any(
                    feature = "google-cloud-bigquery-storage-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-storage-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.storage.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-storage-v1beta2",
                ))]
                pub mod v1beta2 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-storage-v1beta2",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.storage.v1beta2.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-storage-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-storage-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.storage.v1alpha.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-storage-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-storage-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.storage.v1beta1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-bigquery-v2",
                ))]
                include!("bytes_btree_map/google.cloud.bigquery.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-connection-v1",
                feature = "google-cloud-bigquery-connection-v1beta1",
            ))]
            pub mod connection {
                #[cfg(any(
                    feature = "google-cloud-bigquery-connection-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-connection-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.connection.v1beta1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-connection-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-connection-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.connection.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-migration-v2",
                feature = "google-cloud-bigquery-migration-v2alpha",
            ))]
            pub mod migration {
                #[cfg(any(
                    feature = "google-cloud-bigquery-migration-v2",
                ))]
                pub mod v2 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-migration-v2",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.migration.v2.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-migration-v2alpha",
                ))]
                pub mod v2alpha {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-migration-v2alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.migration.v2alpha.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-biglake-v1",
                feature = "google-cloud-bigquery-biglake-v1alpha1",
            ))]
            pub mod biglake {
                #[cfg(any(
                    feature = "google-cloud-bigquery-biglake-v1alpha1",
                ))]
                pub mod v1alpha1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-biglake-v1alpha1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.biglake.v1alpha1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-bigquery-biglake-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-biglake-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.biglake.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-dataexchange-v1beta1",
            ))]
            pub mod dataexchange {
                #[cfg(any(
                    feature = "google-cloud-bigquery-dataexchange-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-dataexchange-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.dataexchange.v1beta1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-bigquery-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-datatransfer-v1",
            ))]
            pub mod datatransfer {
                #[cfg(any(
                    feature = "google-cloud-bigquery-datatransfer-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-datatransfer-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.datatransfer.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-bigquery-reservation-v1",
            ))]
            pub mod reservation {
                #[cfg(any(
                    feature = "google-cloud-bigquery-reservation-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-bigquery-reservation-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.bigquery.reservation.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-speech-v1",
            feature = "google-cloud-speech-v1p1beta1",
            feature = "google-cloud-speech-v2",
        ))]
        pub mod speech {
            #[cfg(any(
                feature = "google-cloud-speech-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-speech-v1",
                ))]
                include!("bytes_btree_map/google.cloud.speech.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-speech-v1p1beta1",
            ))]
            pub mod v1p1beta1 {
                #[cfg(any(
                    feature = "google-cloud-speech-v1p1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.speech.v1p1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-speech-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-speech-v2",
                ))]
                include!("bytes_btree_map/google.cloud.speech.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-apigeeconnect-v1",
        ))]
        pub mod apigeeconnect {
            #[cfg(any(
                feature = "google-cloud-apigeeconnect-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-apigeeconnect-v1",
                ))]
                include!("bytes_btree_map/google.cloud.apigeeconnect.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-datastream-logging-v1",
            feature = "google-cloud-datastream-v1",
            feature = "google-cloud-datastream-v1alpha1",
        ))]
        pub mod datastream {
            #[cfg(any(
                feature = "google-cloud-datastream-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-datastream-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-datastream-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.datastream.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-datastream-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-cloud-datastream-v1alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.datastream.v1alpha1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-datastream-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-datastream-v1",
                ))]
                include!("bytes_btree_map/google.cloud.datastream.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-talent-v4",
            feature = "google-cloud-talent-v4beta1",
        ))]
        pub mod talent {
            #[cfg(any(
                feature = "google-cloud-talent-v4",
            ))]
            pub mod v4 {
                #[cfg(any(
                    feature = "google-cloud-talent-v4",
                ))]
                include!("bytes_btree_map/google.cloud.talent.v4.rs");
            }
            #[cfg(any(
                feature = "google-cloud-talent-v4beta1",
            ))]
            pub mod v4beta1 {
                #[cfg(any(
                    feature = "google-cloud-talent-v4beta1",
                ))]
                include!("bytes_btree_map/google.cloud.talent.v4beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-gkebackup-logging-v1",
            feature = "google-cloud-gkebackup-v1",
        ))]
        pub mod gkebackup {
            #[cfg(any(
                feature = "google-cloud-gkebackup-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-gkebackup-v1",
                ))]
                include!("bytes_btree_map/google.cloud.gkebackup.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-gkebackup-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-gkebackup-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-gkebackup-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkebackup.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
            feature = "google-cloud-gkehub-configmanagement-v1",
            feature = "google-cloud-gkehub-configmanagement-v1alpha",
            feature = "google-cloud-gkehub-configmanagement-v1beta",
            feature = "google-cloud-gkehub-metering-v1alpha",
            feature = "google-cloud-gkehub-metering-v1beta",
            feature = "google-cloud-gkehub-multiclusteringress-v1",
            feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
            feature = "google-cloud-gkehub-multiclusteringress-v1beta",
            feature = "google-cloud-gkehub-servicemesh-v1alpha",
            feature = "google-cloud-gkehub-servicemesh-v1beta",
            feature = "google-cloud-gkehub-v1",
            feature = "google-cloud-gkehub-v1alpha",
            feature = "google-cloud-gkehub-v1beta",
            feature = "google-cloud-gkehub-v1beta1",
        ))]
        pub mod gkehub {
            #[cfg(any(
                feature = "google-cloud-gkehub-multiclusteringress-v1",
                feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
                feature = "google-cloud-gkehub-multiclusteringress-v1beta",
            ))]
            pub mod multiclusteringress {
                #[cfg(any(
                    feature = "google-cloud-gkehub-multiclusteringress-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-multiclusteringress-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.multiclusteringress.v1beta.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.multiclusteringress.v1alpha.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkehub-multiclusteringress-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-multiclusteringress-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.multiclusteringress.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-configmanagement-v1",
                feature = "google-cloud-gkehub-configmanagement-v1alpha",
                feature = "google-cloud-gkehub-configmanagement-v1beta",
            ))]
            pub mod configmanagement {
                #[cfg(any(
                    feature = "google-cloud-gkehub-configmanagement-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-configmanagement-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.configmanagement.v1beta.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkehub-configmanagement-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-configmanagement-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.configmanagement.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkehub-configmanagement-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-configmanagement-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.configmanagement.v1alpha.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-gkehub-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.gkehub.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-metering-v1alpha",
                feature = "google-cloud-gkehub-metering-v1beta",
            ))]
            pub mod metering {
                #[cfg(any(
                    feature = "google-cloud-gkehub-metering-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-metering-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.metering.v1alpha.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkehub-metering-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-metering-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.metering.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-gkehub-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.gkehub.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-servicemesh-v1alpha",
                feature = "google-cloud-gkehub-servicemesh-v1beta",
            ))]
            pub mod servicemesh {
                #[cfg(any(
                    feature = "google-cloud-gkehub-servicemesh-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-servicemesh-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.servicemesh.v1alpha.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkehub-servicemesh-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-servicemesh-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.servicemesh.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
            ))]
            pub mod cloudauditlogging {
                #[cfg(any(
                    feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
                ))]
                pub mod v1alpha {
                    #[cfg(any(
                        feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkehub.cloudauditlogging.v1alpha.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-gkehub-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.gkehub.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-gkehub-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-gkehub-v1",
                ))]
                include!("bytes_btree_map/google.cloud.gkehub.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
            feature = "google-cloud-assuredworkloads-v1",
            feature = "google-cloud-assuredworkloads-v1beta1",
        ))]
        pub mod assuredworkloads {
            #[cfg(any(
                feature = "google-cloud-assuredworkloads-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-assuredworkloads-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.assuredworkloads.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
            ))]
            pub mod regulatoryintercept {
                #[cfg(any(
                    feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
                ))]
                pub mod logging {
                    #[cfg(any(
                        feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
                    ))]
                    pub mod v1 {
                        #[cfg(any(
                            feature = "google-cloud-assuredworkloads-regulatoryintercept-logging-v1",
                        ))]
                        include!("bytes_btree_map/google.cloud.assuredworkloads.regulatoryintercept.logging.v1.rs");
                    }
                }
            }
            #[cfg(any(
                feature = "google-cloud-assuredworkloads-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-assuredworkloads-v1",
                ))]
                include!("bytes_btree_map/google.cloud.assuredworkloads.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-policysimulator-v1",
        ))]
        pub mod policysimulator {
            #[cfg(any(
                feature = "google-cloud-policysimulator-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-policysimulator-v1",
                ))]
                include!("bytes_btree_map/google.cloud.policysimulator.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-vision-v1",
            feature = "google-cloud-vision-v1p1beta1",
            feature = "google-cloud-vision-v1p2beta1",
            feature = "google-cloud-vision-v1p3beta1",
            feature = "google-cloud-vision-v1p4beta1",
        ))]
        pub mod vision {
            #[cfg(any(
                feature = "google-cloud-vision-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-vision-v1",
                ))]
                include!("bytes_btree_map/google.cloud.vision.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-vision-v1p1beta1",
            ))]
            pub mod v1p1beta1 {
                #[cfg(any(
                    feature = "google-cloud-vision-v1p1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.vision.v1p1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-vision-v1p2beta1",
            ))]
            pub mod v1p2beta1 {
                #[cfg(any(
                    feature = "google-cloud-vision-v1p2beta1",
                ))]
                include!("bytes_btree_map/google.cloud.vision.v1p2beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-vision-v1p4beta1",
            ))]
            pub mod v1p4beta1 {
                #[cfg(any(
                    feature = "google-cloud-vision-v1p4beta1",
                ))]
                include!("bytes_btree_map/google.cloud.vision.v1p4beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-vision-v1p3beta1",
            ))]
            pub mod v1p3beta1 {
                #[cfg(any(
                    feature = "google-cloud-vision-v1p3beta1",
                ))]
                include!("bytes_btree_map/google.cloud.vision.v1p3beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-documentai-v1",
            feature = "google-cloud-documentai-v1beta1",
            feature = "google-cloud-documentai-v1beta2",
            feature = "google-cloud-documentai-v1beta3",
        ))]
        pub mod documentai {
            #[cfg(any(
                feature = "google-cloud-documentai-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-documentai-v1",
                ))]
                include!("bytes_btree_map/google.cloud.documentai.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-documentai-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-cloud-documentai-v1beta2",
                ))]
                include!("bytes_btree_map/google.cloud.documentai.v1beta2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-documentai-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-documentai-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.documentai.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-documentai-v1beta3",
            ))]
            pub mod v1beta3 {
                #[cfg(any(
                    feature = "google-cloud-documentai-v1beta3",
                ))]
                include!("bytes_btree_map/google.cloud.documentai.v1beta3.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-aiplatform-logging",
            feature = "google-cloud-aiplatform-v1",
            feature = "google-cloud-aiplatform-v1-schema-predict-instance",
            feature = "google-cloud-aiplatform-v1-schema-predict-params",
            feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
            feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
            feature = "google-cloud-aiplatform-v1beta1",
            feature = "google-cloud-aiplatform-v1beta1-schema",
            feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
            feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
            feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
            feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
        ))]
        pub mod aiplatform {
            #[cfg(any(
                feature = "google-cloud-aiplatform-v1beta1",
                feature = "google-cloud-aiplatform-v1beta1-schema",
                feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-aiplatform-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.aiplatform.v1beta1.rs");
                #[cfg(any(
                    feature = "google-cloud-aiplatform-v1beta1-schema",
                    feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                    feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                    feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                    feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
                ))]
                pub mod schema {
                    #[cfg(any(
                        feature = "google-cloud-aiplatform-v1beta1-schema",
                    ))]
                    include!("bytes_btree_map/google.cloud.aiplatform.v1beta1.schema.rs");
                    #[cfg(any(
                        feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                        feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                        feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                    ))]
                    pub mod predict {
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                        ))]
                        pub mod prediction {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1beta1.schema.predict.prediction.rs");
                        }
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                        ))]
                        pub mod instance {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1beta1.schema.predict.instance.rs");
                        }
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                        ))]
                        pub mod params {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1beta1.schema.predict.params.rs");
                        }
                    }
                    #[cfg(any(
                        feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
                    ))]
                    pub mod trainingjob {
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
                        ))]
                        pub mod definition {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1beta1.schema.trainingjob.definition.rs");
                        }
                    }
                }
            }
            #[cfg(any(
                feature = "google-cloud-aiplatform-v1",
                feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                feature = "google-cloud-aiplatform-v1-schema-predict-params",
                feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-aiplatform-v1",
                ))]
                include!("bytes_btree_map/google.cloud.aiplatform.v1.rs");
                #[cfg(any(
                    feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                    feature = "google-cloud-aiplatform-v1-schema-predict-params",
                    feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                    feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
                ))]
                pub mod schema {
                    #[cfg(any(
                        feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                        feature = "google-cloud-aiplatform-v1-schema-predict-params",
                        feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                    ))]
                    pub mod predict {
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1-schema-predict-params",
                        ))]
                        pub mod params {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1-schema-predict-params",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1.schema.predict.params.rs");
                        }
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                        ))]
                        pub mod prediction {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1.schema.predict.prediction.rs");
                        }
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                        ))]
                        pub mod instance {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1-schema-predict-instance",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1.schema.predict.instance.rs");
                        }
                    }
                    #[cfg(any(
                        feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
                    ))]
                    pub mod trainingjob {
                        #[cfg(any(
                            feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
                        ))]
                        pub mod definition {
                            #[cfg(any(
                                feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
                            ))]
                            include!("bytes_btree_map/google.cloud.aiplatform.v1.schema.trainingjob.definition.rs");
                        }
                    }
                }
            }
            #[cfg(any(
                feature = "google-cloud-aiplatform-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-aiplatform-logging",
                ))]
                include!("bytes_btree_map/google.cloud.aiplatform.logging.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-notebooks-logging-v1",
            feature = "google-cloud-notebooks-v1",
            feature = "google-cloud-notebooks-v1beta1",
            feature = "google-cloud-notebooks-v2",
        ))]
        pub mod notebooks {
            #[cfg(any(
                feature = "google-cloud-notebooks-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-notebooks-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.notebooks.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-notebooks-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-notebooks-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-notebooks-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.notebooks.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-notebooks-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-notebooks-v2",
                ))]
                include!("bytes_btree_map/google.cloud.notebooks.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-notebooks-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-notebooks-v1",
                ))]
                include!("bytes_btree_map/google.cloud.notebooks.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-language-v1",
            feature = "google-cloud-language-v1beta1",
            feature = "google-cloud-language-v1beta2",
            feature = "google-cloud-language-v2",
        ))]
        pub mod language {
            #[cfg(any(
                feature = "google-cloud-language-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-language-v2",
                ))]
                include!("bytes_btree_map/google.cloud.language.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-language-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-cloud-language-v1beta2",
                ))]
                include!("bytes_btree_map/google.cloud.language.v1beta2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-language-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-language-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.language.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-language-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-language-v1",
                ))]
                include!("bytes_btree_map/google.cloud.language.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-secrets-v1beta1",
        ))]
        pub mod secrets {
            #[cfg(any(
                feature = "google-cloud-secrets-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-secrets-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.secrets.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-commerce-consumer-procurement-v1",
            feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
        ))]
        pub mod commerce {
            #[cfg(any(
                feature = "google-cloud-commerce-consumer-procurement-v1",
                feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
            ))]
            pub mod consumer {
                #[cfg(any(
                    feature = "google-cloud-commerce-consumer-procurement-v1",
                    feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                ))]
                pub mod procurement {
                    #[cfg(any(
                        feature = "google-cloud-commerce-consumer-procurement-v1",
                    ))]
                    pub mod v1 {
                        #[cfg(any(
                            feature = "google-cloud-commerce-consumer-procurement-v1",
                        ))]
                        include!("bytes_btree_map/google.cloud.commerce.consumer.procurement.v1.rs");
                    }
                    #[cfg(any(
                        feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                    ))]
                    pub mod v1alpha1 {
                        #[cfg(any(
                            feature = "google-cloud-commerce-consumer-procurement-v1alpha1",
                        ))]
                        include!("bytes_btree_map/google.cloud.commerce.consumer.procurement.v1alpha1.rs");
                    }
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-batch-v1",
            feature = "google-cloud-batch-v1alpha",
        ))]
        pub mod batch {
            #[cfg(any(
                feature = "google-cloud-batch-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-batch-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.batch.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-batch-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-batch-v1",
                ))]
                include!("bytes_btree_map/google.cloud.batch.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-accessapproval-v1",
        ))]
        pub mod accessapproval {
            #[cfg(any(
                feature = "google-cloud-accessapproval-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-accessapproval-v1",
                ))]
                include!("bytes_btree_map/google.cloud.accessapproval.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-resourcesettings-v1",
        ))]
        pub mod resourcesettings {
            #[cfg(any(
                feature = "google-cloud-resourcesettings-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-resourcesettings-v1",
                ))]
                include!("bytes_btree_map/google.cloud.resourcesettings.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-networkmanagement-v1",
            feature = "google-cloud-networkmanagement-v1beta1",
        ))]
        pub mod networkmanagement {
            #[cfg(any(
                feature = "google-cloud-networkmanagement-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-networkmanagement-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.networkmanagement.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-networkmanagement-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-networkmanagement-v1",
                ))]
                include!("bytes_btree_map/google.cloud.networkmanagement.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-dataproc-logging",
            feature = "google-cloud-dataproc-v1",
        ))]
        pub mod dataproc {
            #[cfg(any(
                feature = "google-cloud-dataproc-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-dataproc-logging",
                ))]
                include!("bytes_btree_map/google.cloud.dataproc.logging.rs");
            }
            #[cfg(any(
                feature = "google-cloud-dataproc-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-dataproc-v1",
                ))]
                include!("bytes_btree_map/google.cloud.dataproc.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-dataqna-v1alpha",
        ))]
        pub mod dataqna {
            #[cfg(any(
                feature = "google-cloud-dataqna-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-dataqna-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.dataqna.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-oslogin-common",
            feature = "google-cloud-oslogin-v1",
            feature = "google-cloud-oslogin-v1alpha",
            feature = "google-cloud-oslogin-v1beta",
        ))]
        pub mod oslogin {
            #[cfg(any(
                feature = "google-cloud-oslogin-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-oslogin-v1",
                ))]
                include!("bytes_btree_map/google.cloud.oslogin.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-oslogin-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-oslogin-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.oslogin.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-oslogin-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-oslogin-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.oslogin.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-oslogin-common",
            ))]
            pub mod common {
                #[cfg(any(
                    feature = "google-cloud-oslogin-common",
                ))]
                include!("bytes_btree_map/google.cloud.oslogin.common.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-sensitiveaction-logging-v1",
        ))]
        pub mod sensitiveaction {
            #[cfg(any(
                feature = "google-cloud-sensitiveaction-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-sensitiveaction-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-sensitiveaction-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.sensitiveaction.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-metastore-logging-v1",
            feature = "google-cloud-metastore-v1",
            feature = "google-cloud-metastore-v1alpha",
            feature = "google-cloud-metastore-v1beta",
        ))]
        pub mod metastore {
            #[cfg(any(
                feature = "google-cloud-metastore-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-metastore-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-metastore-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.metastore.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-metastore-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-metastore-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.metastore.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-metastore-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-metastore-v1",
                ))]
                include!("bytes_btree_map/google.cloud.metastore.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-metastore-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-metastore-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.metastore.v1beta.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-config-v1",
        ))]
        pub mod config {
            #[cfg(any(
                feature = "google-cloud-config-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-config-v1",
                ))]
                include!("bytes_btree_map/google.cloud.config.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-tpu-v1",
            feature = "google-cloud-tpu-v2",
            feature = "google-cloud-tpu-v2alpha1",
        ))]
        pub mod tpu {
            #[cfg(any(
                feature = "google-cloud-tpu-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-tpu-v2",
                ))]
                include!("bytes_btree_map/google.cloud.tpu.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-tpu-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-tpu-v1",
                ))]
                include!("bytes_btree_map/google.cloud.tpu.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-tpu-v2alpha1",
            ))]
            pub mod v2alpha1 {
                #[cfg(any(
                    feature = "google-cloud-tpu-v2alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.tpu.v2alpha1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-billing-budgets-v1",
            feature = "google-cloud-billing-budgets-v1beta1",
            feature = "google-cloud-billing-v1",
        ))]
        pub mod billing {
            #[cfg(any(
                feature = "google-cloud-billing-budgets-v1",
                feature = "google-cloud-billing-budgets-v1beta1",
            ))]
            pub mod budgets {
                #[cfg(any(
                    feature = "google-cloud-billing-budgets-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-billing-budgets-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.billing.budgets.v1beta1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-billing-budgets-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-billing-budgets-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.billing.budgets.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-billing-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-billing-v1",
                ))]
                include!("bytes_btree_map/google.cloud.billing.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-location",
        ))]
        pub mod location {
            #[cfg(any(
                feature = "google-cloud-location",
            ))]
            include!("bytes_btree_map/google.cloud.location.rs");
        }
        #[cfg(any(
            feature = "google-cloud-vpcaccess-v1",
        ))]
        pub mod vpcaccess {
            #[cfg(any(
                feature = "google-cloud-vpcaccess-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-vpcaccess-v1",
                ))]
                include!("bytes_btree_map/google.cloud.vpcaccess.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-channel-v1",
        ))]
        pub mod channel {
            #[cfg(any(
                feature = "google-cloud-channel-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-channel-v1",
                ))]
                include!("bytes_btree_map/google.cloud.channel.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-integrations-v1alpha",
        ))]
        pub mod integrations {
            #[cfg(any(
                feature = "google-cloud-integrations-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-integrations-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.integrations.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-stream-logging-v1",
        ))]
        pub mod stream {
            #[cfg(any(
                feature = "google-cloud-stream-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-stream-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-stream-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.stream.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-gdchardwaremanagement-v1alpha",
        ))]
        pub mod gdchardwaremanagement {
            #[cfg(any(
                feature = "google-cloud-gdchardwaremanagement-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-gdchardwaremanagement-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.gdchardwaremanagement.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-domains-v1",
            feature = "google-cloud-domains-v1alpha2",
            feature = "google-cloud-domains-v1beta1",
        ))]
        pub mod domains {
            #[cfg(any(
                feature = "google-cloud-domains-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-domains-v1",
                ))]
                include!("bytes_btree_map/google.cloud.domains.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-domains-v1alpha2",
            ))]
            pub mod v1alpha2 {
                #[cfg(any(
                    feature = "google-cloud-domains-v1alpha2",
                ))]
                include!("bytes_btree_map/google.cloud.domains.v1alpha2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-domains-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-domains-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.domains.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-retail-logging",
            feature = "google-cloud-retail-v2",
            feature = "google-cloud-retail-v2alpha",
            feature = "google-cloud-retail-v2beta",
        ))]
        pub mod retail {
            #[cfg(any(
                feature = "google-cloud-retail-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-retail-v2",
                ))]
                include!("bytes_btree_map/google.cloud.retail.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-retail-v2beta",
            ))]
            pub mod v2beta {
                #[cfg(any(
                    feature = "google-cloud-retail-v2beta",
                ))]
                include!("bytes_btree_map/google.cloud.retail.v2beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-retail-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-retail-logging",
                ))]
                include!("bytes_btree_map/google.cloud.retail.logging.rs");
            }
            #[cfg(any(
                feature = "google-cloud-retail-v2alpha",
            ))]
            pub mod v2alpha {
                #[cfg(any(
                    feature = "google-cloud-retail-v2alpha",
                ))]
                include!("bytes_btree_map/google.cloud.retail.v2alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-gsuiteaddons-logging-v1",
            feature = "google-cloud-gsuiteaddons-v1",
        ))]
        pub mod gsuiteaddons {
            #[cfg(any(
                feature = "google-cloud-gsuiteaddons-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-gsuiteaddons-v1",
                ))]
                include!("bytes_btree_map/google.cloud.gsuiteaddons.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-gsuiteaddons-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-gsuiteaddons-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-gsuiteaddons-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.gsuiteaddons.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-orchestration-airflow-service-v1",
            feature = "google-cloud-orchestration-airflow-service-v1beta1",
        ))]
        pub mod orchestration {
            #[cfg(any(
                feature = "google-cloud-orchestration-airflow-service-v1",
                feature = "google-cloud-orchestration-airflow-service-v1beta1",
            ))]
            pub mod airflow {
                #[cfg(any(
                    feature = "google-cloud-orchestration-airflow-service-v1",
                    feature = "google-cloud-orchestration-airflow-service-v1beta1",
                ))]
                pub mod service {
                    #[cfg(any(
                        feature = "google-cloud-orchestration-airflow-service-v1",
                    ))]
                    pub mod v1 {
                        #[cfg(any(
                            feature = "google-cloud-orchestration-airflow-service-v1",
                        ))]
                        include!("bytes_btree_map/google.cloud.orchestration.airflow.service.v1.rs");
                    }
                    #[cfg(any(
                        feature = "google-cloud-orchestration-airflow-service-v1beta1",
                    ))]
                    pub mod v1beta1 {
                        #[cfg(any(
                            feature = "google-cloud-orchestration-airflow-service-v1beta1",
                        ))]
                        include!("bytes_btree_map/google.cloud.orchestration.airflow.service.v1beta1.rs");
                    }
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-dialogflow-cx-v3",
            feature = "google-cloud-dialogflow-cx-v3beta1",
            feature = "google-cloud-dialogflow-v2",
            feature = "google-cloud-dialogflow-v2beta1",
        ))]
        pub mod dialogflow {
            #[cfg(any(
                feature = "google-cloud-dialogflow-v2beta1",
            ))]
            pub mod v2beta1 {
                #[cfg(any(
                    feature = "google-cloud-dialogflow-v2beta1",
                ))]
                include!("bytes_btree_map/google.cloud.dialogflow.v2beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-dialogflow-cx-v3",
                feature = "google-cloud-dialogflow-cx-v3beta1",
            ))]
            pub mod cx {
                #[cfg(any(
                    feature = "google-cloud-dialogflow-cx-v3beta1",
                ))]
                pub mod v3beta1 {
                    #[cfg(any(
                        feature = "google-cloud-dialogflow-cx-v3beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.dialogflow.cx.v3beta1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-dialogflow-cx-v3",
                ))]
                pub mod v3 {
                    #[cfg(any(
                        feature = "google-cloud-dialogflow-cx-v3",
                    ))]
                    include!("bytes_btree_map/google.cloud.dialogflow.cx.v3.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-dialogflow-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-dialogflow-v2",
                ))]
                include!("bytes_btree_map/google.cloud.dialogflow.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-video-livestream-logging-v1",
            feature = "google-cloud-video-livestream-v1",
            feature = "google-cloud-video-stitcher-v1",
            feature = "google-cloud-video-transcoder-v1",
        ))]
        pub mod video {
            #[cfg(any(
                feature = "google-cloud-video-livestream-logging-v1",
                feature = "google-cloud-video-livestream-v1",
            ))]
            pub mod livestream {
                #[cfg(any(
                    feature = "google-cloud-video-livestream-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-video-livestream-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.video.livestream.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-video-livestream-logging-v1",
                ))]
                pub mod logging {
                    #[cfg(any(
                        feature = "google-cloud-video-livestream-logging-v1",
                    ))]
                    pub mod v1 {
                        #[cfg(any(
                            feature = "google-cloud-video-livestream-logging-v1",
                        ))]
                        include!("bytes_btree_map/google.cloud.video.livestream.logging.v1.rs");
                    }
                }
            }
            #[cfg(any(
                feature = "google-cloud-video-transcoder-v1",
            ))]
            pub mod transcoder {
                #[cfg(any(
                    feature = "google-cloud-video-transcoder-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-video-transcoder-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.video.transcoder.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-video-stitcher-v1",
            ))]
            pub mod stitcher {
                #[cfg(any(
                    feature = "google-cloud-video-stitcher-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-video-stitcher-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.video.stitcher.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-workflows-executions-v1",
            feature = "google-cloud-workflows-executions-v1beta",
            feature = "google-cloud-workflows-type",
            feature = "google-cloud-workflows-v1",
            feature = "google-cloud-workflows-v1beta",
        ))]
        pub mod workflows {
            #[cfg(any(
                feature = "google-cloud-workflows-executions-v1",
                feature = "google-cloud-workflows-executions-v1beta",
            ))]
            pub mod executions {
                #[cfg(any(
                    feature = "google-cloud-workflows-executions-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-workflows-executions-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.workflows.executions.v1beta.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-workflows-executions-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-workflows-executions-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.workflows.executions.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-workflows-type",
            ))]
            pub mod r#type {
                #[cfg(any(
                    feature = "google-cloud-workflows-type",
                ))]
                include!("bytes_btree_map/google.cloud.workflows.r#type.rs");
            }
            #[cfg(any(
                feature = "google-cloud-workflows-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-workflows-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.workflows.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-workflows-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-workflows-v1",
                ))]
                include!("bytes_btree_map/google.cloud.workflows.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
            feature = "google-cloud-paymentgateway-issuerswitch-v1",
        ))]
        pub mod paymentgateway {
            #[cfg(any(
                feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
                feature = "google-cloud-paymentgateway-issuerswitch-v1",
            ))]
            pub mod issuerswitch {
                #[cfg(any(
                    feature = "google-cloud-paymentgateway-issuerswitch-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-paymentgateway-issuerswitch-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.paymentgateway.issuerswitch.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
                ))]
                pub mod accountmanager {
                    #[cfg(any(
                        feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
                    ))]
                    pub mod v1 {
                        #[cfg(any(
                            feature = "google-cloud-paymentgateway-issuerswitch-accountmanager-v1",
                        ))]
                        include!("bytes_btree_map/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.rs");
                    }
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-securityposture-v1",
        ))]
        pub mod securityposture {
            #[cfg(any(
                feature = "google-cloud-securityposture-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-securityposture-v1",
                ))]
                include!("bytes_btree_map/google.cloud.securityposture.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-netapp-v1",
        ))]
        pub mod netapp {
            #[cfg(any(
                feature = "google-cloud-netapp-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-netapp-v1",
                ))]
                include!("bytes_btree_map/google.cloud.netapp.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-secretmanager-logging-v1",
            feature = "google-cloud-secretmanager-v1",
            feature = "google-cloud-secretmanager-v1beta2",
        ))]
        pub mod secretmanager {
            #[cfg(any(
                feature = "google-cloud-secretmanager-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-secretmanager-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-secretmanager-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.secretmanager.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-secretmanager-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-secretmanager-v1",
                ))]
                include!("bytes_btree_map/google.cloud.secretmanager.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-secretmanager-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-cloud-secretmanager-v1beta2",
                ))]
                include!("bytes_btree_map/google.cloud.secretmanager.v1beta2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-timeseriesinsights-v1",
        ))]
        pub mod timeseriesinsights {
            #[cfg(any(
                feature = "google-cloud-timeseriesinsights-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-timeseriesinsights-v1",
                ))]
                include!("bytes_btree_map/google.cloud.timeseriesinsights.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-webrisk-v1",
            feature = "google-cloud-webrisk-v1beta1",
        ))]
        pub mod webrisk {
            #[cfg(any(
                feature = "google-cloud-webrisk-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-webrisk-v1",
                ))]
                include!("bytes_btree_map/google.cloud.webrisk.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-webrisk-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-webrisk-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.webrisk.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-dataform-logging-v1",
            feature = "google-cloud-dataform-v1alpha2",
            feature = "google-cloud-dataform-v1beta1",
        ))]
        pub mod dataform {
            #[cfg(any(
                feature = "google-cloud-dataform-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-dataform-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-dataform-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.dataform.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-dataform-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-dataform-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.dataform.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-dataform-v1alpha2",
            ))]
            pub mod v1alpha2 {
                #[cfg(any(
                    feature = "google-cloud-dataform-v1alpha2",
                ))]
                include!("bytes_btree_map/google.cloud.dataform.v1alpha2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-securitycenter-settings-v1beta1",
            feature = "google-cloud-securitycenter-v1",
            feature = "google-cloud-securitycenter-v1beta1",
            feature = "google-cloud-securitycenter-v1p1beta1",
            feature = "google-cloud-securitycenter-v2",
        ))]
        pub mod securitycenter {
            #[cfg(any(
                feature = "google-cloud-securitycenter-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-securitycenter-v2",
                ))]
                include!("bytes_btree_map/google.cloud.securitycenter.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-securitycenter-settings-v1beta1",
            ))]
            pub mod settings {
                #[cfg(any(
                    feature = "google-cloud-securitycenter-settings-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-securitycenter-settings-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.securitycenter.settings.v1beta1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-securitycenter-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-securitycenter-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.securitycenter.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-securitycenter-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-securitycenter-v1",
                ))]
                include!("bytes_btree_map/google.cloud.securitycenter.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-securitycenter-v1p1beta1",
            ))]
            pub mod v1p1beta1 {
                #[cfg(any(
                    feature = "google-cloud-securitycenter-v1p1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.securitycenter.v1p1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-lifesciences-v2beta",
        ))]
        pub mod lifesciences {
            #[cfg(any(
                feature = "google-cloud-lifesciences-v2beta",
            ))]
            pub mod v2beta {
                #[cfg(any(
                    feature = "google-cloud-lifesciences-v2beta",
                ))]
                include!("bytes_btree_map/google.cloud.lifesciences.v2beta.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-texttospeech-v1",
            feature = "google-cloud-texttospeech-v1beta1",
        ))]
        pub mod texttospeech {
            #[cfg(any(
                feature = "google-cloud-texttospeech-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-texttospeech-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.texttospeech.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-texttospeech-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-texttospeech-v1",
                ))]
                include!("bytes_btree_map/google.cloud.texttospeech.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-cloudcontrolspartner-v1",
            feature = "google-cloud-cloudcontrolspartner-v1beta",
        ))]
        pub mod cloudcontrolspartner {
            #[cfg(any(
                feature = "google-cloud-cloudcontrolspartner-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-cloudcontrolspartner-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.cloudcontrolspartner.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-cloudcontrolspartner-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-cloudcontrolspartner-v1",
                ))]
                include!("bytes_btree_map/google.cloud.cloudcontrolspartner.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-asset-v1",
            feature = "google-cloud-asset-v1p1beta1",
            feature = "google-cloud-asset-v1p2beta1",
            feature = "google-cloud-asset-v1p5beta1",
            feature = "google-cloud-asset-v1p7beta1",
        ))]
        pub mod asset {
            #[cfg(any(
                feature = "google-cloud-asset-v1p1beta1",
            ))]
            pub mod v1p1beta1 {
                #[cfg(any(
                    feature = "google-cloud-asset-v1p1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.asset.v1p1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-asset-v1p2beta1",
            ))]
            pub mod v1p2beta1 {
                #[cfg(any(
                    feature = "google-cloud-asset-v1p2beta1",
                ))]
                include!("bytes_btree_map/google.cloud.asset.v1p2beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-asset-v1p5beta1",
            ))]
            pub mod v1p5beta1 {
                #[cfg(any(
                    feature = "google-cloud-asset-v1p5beta1",
                ))]
                include!("bytes_btree_map/google.cloud.asset.v1p5beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-asset-v1p7beta1",
            ))]
            pub mod v1p7beta1 {
                #[cfg(any(
                    feature = "google-cloud-asset-v1p7beta1",
                ))]
                include!("bytes_btree_map/google.cloud.asset.v1p7beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-asset-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-asset-v1",
                ))]
                include!("bytes_btree_map/google.cloud.asset.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-redis-cluster-v1",
            feature = "google-cloud-redis-cluster-v1beta1",
            feature = "google-cloud-redis-v1",
            feature = "google-cloud-redis-v1beta1",
        ))]
        pub mod redis {
            #[cfg(any(
                feature = "google-cloud-redis-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-redis-v1",
                ))]
                include!("bytes_btree_map/google.cloud.redis.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-redis-cluster-v1",
                feature = "google-cloud-redis-cluster-v1beta1",
            ))]
            pub mod cluster {
                #[cfg(any(
                    feature = "google-cloud-redis-cluster-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-redis-cluster-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.redis.cluster.v1beta1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-redis-cluster-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-redis-cluster-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.redis.cluster.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-redis-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-redis-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.redis.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-gkeconnect-gateway-v1",
            feature = "google-cloud-gkeconnect-gateway-v1beta1",
        ))]
        pub mod gkeconnect {
            #[cfg(any(
                feature = "google-cloud-gkeconnect-gateway-v1",
                feature = "google-cloud-gkeconnect-gateway-v1beta1",
            ))]
            pub mod gateway {
                #[cfg(any(
                    feature = "google-cloud-gkeconnect-gateway-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-gkeconnect-gateway-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkeconnect.gateway.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-gkeconnect-gateway-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-gkeconnect-gateway-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.gkeconnect.gateway.v1beta1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-networkservices-v1",
            feature = "google-cloud-networkservices-v1beta1",
        ))]
        pub mod networkservices {
            #[cfg(any(
                feature = "google-cloud-networkservices-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-networkservices-v1",
                ))]
                include!("bytes_btree_map/google.cloud.networkservices.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-networkservices-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-networkservices-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.networkservices.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-datacatalog-lineage-v1",
            feature = "google-cloud-datacatalog-v1",
            feature = "google-cloud-datacatalog-v1beta1",
        ))]
        pub mod datacatalog {
            #[cfg(any(
                feature = "google-cloud-datacatalog-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-datacatalog-v1",
                ))]
                include!("bytes_btree_map/google.cloud.datacatalog.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-datacatalog-lineage-v1",
            ))]
            pub mod lineage {
                #[cfg(any(
                    feature = "google-cloud-datacatalog-lineage-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-datacatalog-lineage-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.datacatalog.lineage.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-datacatalog-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-datacatalog-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.datacatalog.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-discoveryengine-logging",
            feature = "google-cloud-discoveryengine-v1",
            feature = "google-cloud-discoveryengine-v1alpha",
            feature = "google-cloud-discoveryengine-v1beta",
        ))]
        pub mod discoveryengine {
            #[cfg(any(
                feature = "google-cloud-discoveryengine-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-discoveryengine-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.discoveryengine.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-discoveryengine-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-discoveryengine-logging",
                ))]
                include!("bytes_btree_map/google.cloud.discoveryengine.logging.rs");
            }
            #[cfg(any(
                feature = "google-cloud-discoveryengine-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-discoveryengine-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.discoveryengine.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-discoveryengine-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-discoveryengine-v1",
                ))]
                include!("bytes_btree_map/google.cloud.discoveryengine.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-orgpolicy-v1",
            feature = "google-cloud-orgpolicy-v2",
        ))]
        pub mod orgpolicy {
            #[cfg(any(
                feature = "google-cloud-orgpolicy-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-orgpolicy-v1",
                ))]
                include!("bytes_btree_map/google.cloud.orgpolicy.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-orgpolicy-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-orgpolicy-v2",
                ))]
                include!("bytes_btree_map/google.cloud.orgpolicy.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-servicedirectory-v1",
            feature = "google-cloud-servicedirectory-v1beta1",
        ))]
        pub mod servicedirectory {
            #[cfg(any(
                feature = "google-cloud-servicedirectory-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-servicedirectory-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.servicedirectory.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-servicedirectory-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-servicedirectory-v1",
                ))]
                include!("bytes_btree_map/google.cloud.servicedirectory.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-certificatemanager-logging-v1",
            feature = "google-cloud-certificatemanager-v1",
        ))]
        pub mod certificatemanager {
            #[cfg(any(
                feature = "google-cloud-certificatemanager-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-certificatemanager-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-certificatemanager-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.certificatemanager.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-certificatemanager-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-certificatemanager-v1",
                ))]
                include!("bytes_btree_map/google.cloud.certificatemanager.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-binaryauthorization-v1",
            feature = "google-cloud-binaryauthorization-v1beta1",
        ))]
        pub mod binaryauthorization {
            #[cfg(any(
                feature = "google-cloud-binaryauthorization-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-binaryauthorization-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.binaryauthorization.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-binaryauthorization-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-binaryauthorization-v1",
                ))]
                include!("bytes_btree_map/google.cloud.binaryauthorization.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-connectors-v1",
        ))]
        pub mod connectors {
            #[cfg(any(
                feature = "google-cloud-connectors-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-connectors-v1",
                ))]
                include!("bytes_btree_map/google.cloud.connectors.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-osconfig-agentendpoint-v1",
            feature = "google-cloud-osconfig-agentendpoint-v1beta",
            feature = "google-cloud-osconfig-logging",
            feature = "google-cloud-osconfig-v1",
            feature = "google-cloud-osconfig-v1alpha",
            feature = "google-cloud-osconfig-v1beta",
        ))]
        pub mod osconfig {
            #[cfg(any(
                feature = "google-cloud-osconfig-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-osconfig-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.osconfig.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-osconfig-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-osconfig-v1",
                ))]
                include!("bytes_btree_map/google.cloud.osconfig.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-osconfig-agentendpoint-v1",
                feature = "google-cloud-osconfig-agentendpoint-v1beta",
            ))]
            pub mod agentendpoint {
                #[cfg(any(
                    feature = "google-cloud-osconfig-agentendpoint-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-osconfig-agentendpoint-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.osconfig.agentendpoint.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-osconfig-agentendpoint-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-cloud-osconfig-agentendpoint-v1beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.osconfig.agentendpoint.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-osconfig-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-osconfig-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.osconfig.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-osconfig-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-osconfig-logging",
                ))]
                include!("bytes_btree_map/google.cloud.osconfig.logging.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-scheduler-v1",
            feature = "google-cloud-scheduler-v1beta1",
        ))]
        pub mod scheduler {
            #[cfg(any(
                feature = "google-cloud-scheduler-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-scheduler-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.scheduler.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-scheduler-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-scheduler-v1",
                ))]
                include!("bytes_btree_map/google.cloud.scheduler.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-recommender-logging-v1",
            feature = "google-cloud-recommender-logging-v1beta1",
            feature = "google-cloud-recommender-v1",
            feature = "google-cloud-recommender-v1beta1",
        ))]
        pub mod recommender {
            #[cfg(any(
                feature = "google-cloud-recommender-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-recommender-v1",
                ))]
                include!("bytes_btree_map/google.cloud.recommender.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-recommender-logging-v1",
                feature = "google-cloud-recommender-logging-v1beta1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-recommender-logging-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-recommender-logging-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.recommender.logging.v1beta1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-recommender-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-recommender-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.recommender.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-recommender-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-recommender-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.recommender.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-sql-v1",
            feature = "google-cloud-sql-v1beta4",
        ))]
        pub mod sql {
            #[cfg(any(
                feature = "google-cloud-sql-v1beta4",
            ))]
            pub mod v1beta4 {
                #[cfg(any(
                    feature = "google-cloud-sql-v1beta4",
                ))]
                include!("bytes_btree_map/google.cloud.sql.v1beta4.rs");
            }
            #[cfg(any(
                feature = "google-cloud-sql-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-sql-v1",
                ))]
                include!("bytes_btree_map/google.cloud.sql.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-iap-v1",
            feature = "google-cloud-iap-v1beta1",
        ))]
        pub mod iap {
            #[cfg(any(
                feature = "google-cloud-iap-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-iap-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.iap.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-iap-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-iap-v1",
                ))]
                include!("bytes_btree_map/google.cloud.iap.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-runtimeconfig-v1beta1",
        ))]
        pub mod runtimeconfig {
            #[cfg(any(
                feature = "google-cloud-runtimeconfig-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-runtimeconfig-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.runtimeconfig.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-workstations-logging-v1",
            feature = "google-cloud-workstations-v1",
            feature = "google-cloud-workstations-v1beta",
        ))]
        pub mod workstations {
            #[cfg(any(
                feature = "google-cloud-workstations-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-workstations-v1",
                ))]
                include!("bytes_btree_map/google.cloud.workstations.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-workstations-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-workstations-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-workstations-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.workstations.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-workstations-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-workstations-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.workstations.v1beta.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-tasks-v2",
            feature = "google-cloud-tasks-v2beta2",
            feature = "google-cloud-tasks-v2beta3",
        ))]
        pub mod tasks {
            #[cfg(any(
                feature = "google-cloud-tasks-v2beta3",
            ))]
            pub mod v2beta3 {
                #[cfg(any(
                    feature = "google-cloud-tasks-v2beta3",
                ))]
                include!("bytes_btree_map/google.cloud.tasks.v2beta3.rs");
            }
            #[cfg(any(
                feature = "google-cloud-tasks-v2beta2",
            ))]
            pub mod v2beta2 {
                #[cfg(any(
                    feature = "google-cloud-tasks-v2beta2",
                ))]
                include!("bytes_btree_map/google.cloud.tasks.v2beta2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-tasks-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-tasks-v2",
                ))]
                include!("bytes_btree_map/google.cloud.tasks.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-developerconnect-v1",
        ))]
        pub mod developerconnect {
            #[cfg(any(
                feature = "google-cloud-developerconnect-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-developerconnect-v1",
                ))]
                include!("bytes_btree_map/google.cloud.developerconnect.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-security-privateca-v1",
            feature = "google-cloud-security-privateca-v1beta1",
            feature = "google-cloud-security-publicca-v1",
            feature = "google-cloud-security-publicca-v1beta1",
        ))]
        pub mod security {
            #[cfg(any(
                feature = "google-cloud-security-privateca-v1",
                feature = "google-cloud-security-privateca-v1beta1",
            ))]
            pub mod privateca {
                #[cfg(any(
                    feature = "google-cloud-security-privateca-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-security-privateca-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.security.privateca.v1beta1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-security-privateca-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-security-privateca-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.security.privateca.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-security-publicca-v1",
                feature = "google-cloud-security-publicca-v1beta1",
            ))]
            pub mod publicca {
                #[cfg(any(
                    feature = "google-cloud-security-publicca-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-security-publicca-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.security.publicca.v1.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-security-publicca-v1beta1",
                ))]
                pub mod v1beta1 {
                    #[cfg(any(
                        feature = "google-cloud-security-publicca-v1beta1",
                    ))]
                    include!("bytes_btree_map/google.cloud.security.publicca.v1beta1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-apphub-v1",
        ))]
        pub mod apphub {
            #[cfg(any(
                feature = "google-cloud-apphub-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-apphub-v1",
                ))]
                include!("bytes_btree_map/google.cloud.apphub.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-memcache-v1",
            feature = "google-cloud-memcache-v1beta2",
        ))]
        pub mod memcache {
            #[cfg(any(
                feature = "google-cloud-memcache-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-memcache-v1",
                ))]
                include!("bytes_btree_map/google.cloud.memcache.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-memcache-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-cloud-memcache-v1beta2",
                ))]
                include!("bytes_btree_map/google.cloud.memcache.v1beta2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-rapidmigrationassessment-v1",
        ))]
        pub mod rapidmigrationassessment {
            #[cfg(any(
                feature = "google-cloud-rapidmigrationassessment-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-rapidmigrationassessment-v1",
                ))]
                include!("bytes_btree_map/google.cloud.rapidmigrationassessment.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-resourcemanager-v2",
            feature = "google-cloud-resourcemanager-v3",
        ))]
        pub mod resourcemanager {
            #[cfg(any(
                feature = "google-cloud-resourcemanager-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-resourcemanager-v2",
                ))]
                include!("bytes_btree_map/google.cloud.resourcemanager.v2.rs");
            }
            #[cfg(any(
                feature = "google-cloud-resourcemanager-v3",
            ))]
            pub mod v3 {
                #[cfg(any(
                    feature = "google-cloud-resourcemanager-v3",
                ))]
                include!("bytes_btree_map/google.cloud.resourcemanager.v3.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-servicehealth-logging-v1",
            feature = "google-cloud-servicehealth-v1",
        ))]
        pub mod servicehealth {
            #[cfg(any(
                feature = "google-cloud-servicehealth-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-servicehealth-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-servicehealth-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.servicehealth.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-servicehealth-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-servicehealth-v1",
                ))]
                include!("bytes_btree_map/google.cloud.servicehealth.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-cloudsetup-logging-v1",
        ))]
        pub mod cloudsetup {
            #[cfg(any(
                feature = "google-cloud-cloudsetup-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-cloudsetup-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-cloudsetup-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.cloudsetup.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-blockchainnodeengine-v1",
        ))]
        pub mod blockchainnodeengine {
            #[cfg(any(
                feature = "google-cloud-blockchainnodeengine-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-blockchainnodeengine-v1",
                ))]
                include!("bytes_btree_map/google.cloud.blockchainnodeengine.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-advisorynotifications-v1",
        ))]
        pub mod advisorynotifications {
            #[cfg(any(
                feature = "google-cloud-advisorynotifications-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-advisorynotifications-v1",
                ))]
                include!("bytes_btree_map/google.cloud.advisorynotifications.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-apigeeregistry-v1",
        ))]
        pub mod apigeeregistry {
            #[cfg(any(
                feature = "google-cloud-apigeeregistry-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-apigeeregistry-v1",
                ))]
                include!("bytes_btree_map/google.cloud.apigeeregistry.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-privatecatalog-v1beta1",
        ))]
        pub mod privatecatalog {
            #[cfg(any(
                feature = "google-cloud-privatecatalog-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-privatecatalog-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.privatecatalog.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-compute-v1",
            feature = "google-cloud-compute-v1small",
        ))]
        pub mod compute {
            #[cfg(any(
                feature = "google-cloud-compute-v1small",
            ))]
            pub mod v1small {
                #[cfg(any(
                    feature = "google-cloud-compute-v1small",
                ))]
                include!("bytes_btree_map/google.cloud.compute.v1small.rs");
            }
            #[cfg(any(
                feature = "google-cloud-compute-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-compute-v1",
                ))]
                include!("bytes_btree_map/google.cloud.compute.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-mediatranslation-v1alpha1",
            feature = "google-cloud-mediatranslation-v1beta1",
        ))]
        pub mod mediatranslation {
            #[cfg(any(
                feature = "google-cloud-mediatranslation-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-cloud-mediatranslation-v1alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.mediatranslation.v1alpha1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-mediatranslation-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-mediatranslation-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.mediatranslation.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-backupdr-logging-v1",
            feature = "google-cloud-backupdr-v1",
        ))]
        pub mod backupdr {
            #[cfg(any(
                feature = "google-cloud-backupdr-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-backupdr-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-backupdr-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.backupdr.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-backupdr-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-backupdr-v1",
                ))]
                include!("bytes_btree_map/google.cloud.backupdr.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-recaptchaenterprise-v1",
            feature = "google-cloud-recaptchaenterprise-v1beta1",
        ))]
        pub mod recaptchaenterprise {
            #[cfg(any(
                feature = "google-cloud-recaptchaenterprise-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-recaptchaenterprise-v1",
                ))]
                include!("bytes_btree_map/google.cloud.recaptchaenterprise.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-recaptchaenterprise-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-recaptchaenterprise-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.recaptchaenterprise.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-apihub-v1",
        ))]
        pub mod apihub {
            #[cfg(any(
                feature = "google-cloud-apihub-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-apihub-v1",
                ))]
                include!("bytes_btree_map/google.cloud.apihub.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-telcoautomation-v1",
            feature = "google-cloud-telcoautomation-v1alpha1",
        ))]
        pub mod telcoautomation {
            #[cfg(any(
                feature = "google-cloud-telcoautomation-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-cloud-telcoautomation-v1alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.telcoautomation.v1alpha1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-telcoautomation-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-telcoautomation-v1",
                ))]
                include!("bytes_btree_map/google.cloud.telcoautomation.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-networksecurity-v1",
            feature = "google-cloud-networksecurity-v1beta1",
        ))]
        pub mod networksecurity {
            #[cfg(any(
                feature = "google-cloud-networksecurity-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-networksecurity-v1",
                ))]
                include!("bytes_btree_map/google.cloud.networksecurity.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-networksecurity-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-networksecurity-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.networksecurity.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-shell-v1",
        ))]
        pub mod shell {
            #[cfg(any(
                feature = "google-cloud-shell-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-shell-v1",
                ))]
                include!("bytes_btree_map/google.cloud.shell.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-common",
        ))]
        pub mod common {
            #[cfg(any(
                feature = "google-cloud-common",
            ))]
            include!("bytes_btree_map/google.cloud.common.rs");
        }
        #[cfg(any(
            feature = "google-cloud-healthcare-logging",
        ))]
        pub mod healthcare {
            #[cfg(any(
                feature = "google-cloud-healthcare-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-healthcare-logging",
                ))]
                include!("bytes_btree_map/google.cloud.healthcare.logging.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-managedidentities-v1",
            feature = "google-cloud-managedidentities-v1beta1",
        ))]
        pub mod managedidentities {
            #[cfg(any(
                feature = "google-cloud-managedidentities-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-managedidentities-v1",
                ))]
                include!("bytes_btree_map/google.cloud.managedidentities.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-managedidentities-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-managedidentities-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.managedidentities.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-identitytoolkit-logging",
            feature = "google-cloud-identitytoolkit-v2",
        ))]
        pub mod identitytoolkit {
            #[cfg(any(
                feature = "google-cloud-identitytoolkit-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-identitytoolkit-logging",
                ))]
                include!("bytes_btree_map/google.cloud.identitytoolkit.logging.rs");
            }
            #[cfg(any(
                feature = "google-cloud-identitytoolkit-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-identitytoolkit-v2",
                ))]
                include!("bytes_btree_map/google.cloud.identitytoolkit.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-enterpriseknowledgegraph-v1",
        ))]
        pub mod enterpriseknowledgegraph {
            #[cfg(any(
                feature = "google-cloud-enterpriseknowledgegraph-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-enterpriseknowledgegraph-v1",
                ))]
                include!("bytes_btree_map/google.cloud.enterpriseknowledgegraph.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-networkconnectivity-v1",
            feature = "google-cloud-networkconnectivity-v1alpha1",
        ))]
        pub mod networkconnectivity {
            #[cfg(any(
                feature = "google-cloud-networkconnectivity-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-cloud-networkconnectivity-v1alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.networkconnectivity.v1alpha1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-networkconnectivity-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-networkconnectivity-v1",
                ))]
                include!("bytes_btree_map/google.cloud.networkconnectivity.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-audit",
        ))]
        pub mod audit {
            #[cfg(any(
                feature = "google-cloud-audit",
            ))]
            include!("bytes_btree_map/google.cloud.audit.rs");
        }
        #[cfg(any(
            feature = "google-cloud-contentwarehouse-v1",
        ))]
        pub mod contentwarehouse {
            #[cfg(any(
                feature = "google-cloud-contentwarehouse-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-contentwarehouse-v1",
                ))]
                include!("bytes_btree_map/google.cloud.contentwarehouse.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-clouddms-logging-v1",
            feature = "google-cloud-clouddms-v1",
        ))]
        pub mod clouddms {
            #[cfg(any(
                feature = "google-cloud-clouddms-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-clouddms-v1",
                ))]
                include!("bytes_btree_map/google.cloud.clouddms.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-clouddms-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-clouddms-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-clouddms-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.clouddms.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-run-v2",
        ))]
        pub mod run {
            #[cfg(any(
                feature = "google-cloud-run-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-run-v2",
                ))]
                include!("bytes_btree_map/google.cloud.run.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-parallelstore-v1beta",
        ))]
        pub mod parallelstore {
            #[cfg(any(
                feature = "google-cloud-parallelstore-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-parallelstore-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.parallelstore.v1beta.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-dataplex-v1",
        ))]
        pub mod dataplex {
            #[cfg(any(
                feature = "google-cloud-dataplex-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-dataplex-v1",
                ))]
                include!("bytes_btree_map/google.cloud.dataplex.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-confidentialcomputing-v1",
            feature = "google-cloud-confidentialcomputing-v1alpha1",
        ))]
        pub mod confidentialcomputing {
            #[cfg(any(
                feature = "google-cloud-confidentialcomputing-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-cloud-confidentialcomputing-v1alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.confidentialcomputing.v1alpha1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-confidentialcomputing-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-confidentialcomputing-v1",
                ))]
                include!("bytes_btree_map/google.cloud.confidentialcomputing.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-automl-v1",
            feature = "google-cloud-automl-v1beta1",
        ))]
        pub mod automl {
            #[cfg(any(
                feature = "google-cloud-automl-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-automl-v1",
                ))]
                include!("bytes_btree_map/google.cloud.automl.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-automl-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-automl-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.automl.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-vmmigration-v1",
        ))]
        pub mod vmmigration {
            #[cfg(any(
                feature = "google-cloud-vmmigration-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-vmmigration-v1",
                ))]
                include!("bytes_btree_map/google.cloud.vmmigration.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-gkemulticloud-v1",
        ))]
        pub mod gkemulticloud {
            #[cfg(any(
                feature = "google-cloud-gkemulticloud-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-gkemulticloud-v1",
                ))]
                include!("bytes_btree_map/google.cloud.gkemulticloud.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-datapipelines-logging-v1",
        ))]
        pub mod datapipelines {
            #[cfg(any(
                feature = "google-cloud-datapipelines-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-datapipelines-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-datapipelines-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.datapipelines.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-apigateway-v1",
        ))]
        pub mod apigateway {
            #[cfg(any(
                feature = "google-cloud-apigateway-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-apigateway-v1",
                ))]
                include!("bytes_btree_map/google.cloud.apigateway.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-vmwareengine-v1",
        ))]
        pub mod vmwareengine {
            #[cfg(any(
                feature = "google-cloud-vmwareengine-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-vmwareengine-v1",
                ))]
                include!("bytes_btree_map/google.cloud.vmwareengine.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-ids-logging-v1",
            feature = "google-cloud-ids-v1",
        ))]
        pub mod ids {
            #[cfg(any(
                feature = "google-cloud-ids-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-ids-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-ids-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.ids.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-ids-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-ids-v1",
                ))]
                include!("bytes_btree_map/google.cloud.ids.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-websecurityscanner-v1",
            feature = "google-cloud-websecurityscanner-v1alpha",
            feature = "google-cloud-websecurityscanner-v1beta",
        ))]
        pub mod websecurityscanner {
            #[cfg(any(
                feature = "google-cloud-websecurityscanner-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-cloud-websecurityscanner-v1alpha",
                ))]
                include!("bytes_btree_map/google.cloud.websecurityscanner.v1alpha.rs");
            }
            #[cfg(any(
                feature = "google-cloud-websecurityscanner-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-cloud-websecurityscanner-v1beta",
                ))]
                include!("bytes_btree_map/google.cloud.websecurityscanner.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-cloud-websecurityscanner-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-websecurityscanner-v1",
                ))]
                include!("bytes_btree_map/google.cloud.websecurityscanner.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-translation-v3",
            feature = "google-cloud-translation-v3beta1",
        ))]
        pub mod translation {
            #[cfg(any(
                feature = "google-cloud-translation-v3",
            ))]
            pub mod v3 {
                #[cfg(any(
                    feature = "google-cloud-translation-v3",
                ))]
                include!("bytes_btree_map/google.cloud.translation.v3.rs");
            }
            #[cfg(any(
                feature = "google-cloud-translation-v3beta1",
            ))]
            pub mod v3beta1 {
                #[cfg(any(
                    feature = "google-cloud-translation-v3beta1",
                ))]
                include!("bytes_btree_map/google.cloud.translation.v3beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-edgenetwork-v1",
        ))]
        pub mod edgenetwork {
            #[cfg(any(
                feature = "google-cloud-edgenetwork-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-edgenetwork-v1",
                ))]
                include!("bytes_btree_map/google.cloud.edgenetwork.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-baremetalsolution-v2",
        ))]
        pub mod baremetalsolution {
            #[cfg(any(
                feature = "google-cloud-baremetalsolution-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-cloud-baremetalsolution-v2",
                ))]
                include!("bytes_btree_map/google.cloud.baremetalsolution.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-kms-inventory-v1",
            feature = "google-cloud-kms-logging-v1",
            feature = "google-cloud-kms-v1",
        ))]
        pub mod kms {
            #[cfg(any(
                feature = "google-cloud-kms-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-kms-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-kms-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.kms.logging.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-kms-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-kms-v1",
                ))]
                include!("bytes_btree_map/google.cloud.kms.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-kms-inventory-v1",
            ))]
            pub mod inventory {
                #[cfg(any(
                    feature = "google-cloud-kms-inventory-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-kms-inventory-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.kms.inventory.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-contactcenterinsights-v1",
        ))]
        pub mod contactcenterinsights {
            #[cfg(any(
                feature = "google-cloud-contactcenterinsights-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-contactcenterinsights-v1",
                ))]
                include!("bytes_btree_map/google.cloud.contactcenterinsights.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-policytroubleshooter-iam-v3",
            feature = "google-cloud-policytroubleshooter-iam-v3beta",
            feature = "google-cloud-policytroubleshooter-v1",
        ))]
        pub mod policytroubleshooter {
            #[cfg(any(
                feature = "google-cloud-policytroubleshooter-iam-v3",
                feature = "google-cloud-policytroubleshooter-iam-v3beta",
            ))]
            pub mod iam {
                #[cfg(any(
                    feature = "google-cloud-policytroubleshooter-iam-v3beta",
                ))]
                pub mod v3beta {
                    #[cfg(any(
                        feature = "google-cloud-policytroubleshooter-iam-v3beta",
                    ))]
                    include!("bytes_btree_map/google.cloud.policytroubleshooter.iam.v3beta.rs");
                }
                #[cfg(any(
                    feature = "google-cloud-policytroubleshooter-iam-v3",
                ))]
                pub mod v3 {
                    #[cfg(any(
                        feature = "google-cloud-policytroubleshooter-iam-v3",
                    ))]
                    include!("bytes_btree_map/google.cloud.policytroubleshooter.iam.v3.rs");
                }
            }
            #[cfg(any(
                feature = "google-cloud-policytroubleshooter-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-policytroubleshooter-v1",
                ))]
                include!("bytes_btree_map/google.cloud.policytroubleshooter.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-essentialcontacts-v1",
        ))]
        pub mod essentialcontacts {
            #[cfg(any(
                feature = "google-cloud-essentialcontacts-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-essentialcontacts-v1",
                ))]
                include!("bytes_btree_map/google.cloud.essentialcontacts.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-networkanalyzer-logging-v1",
        ))]
        pub mod networkanalyzer {
            #[cfg(any(
                feature = "google-cloud-networkanalyzer-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-networkanalyzer-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-networkanalyzer-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.networkanalyzer.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-saasaccelerator-management-logs-v1",
        ))]
        pub mod saasaccelerator {
            #[cfg(any(
                feature = "google-cloud-saasaccelerator-management-logs-v1",
            ))]
            pub mod management {
                #[cfg(any(
                    feature = "google-cloud-saasaccelerator-management-logs-v1",
                ))]
                pub mod logs {
                    #[cfg(any(
                        feature = "google-cloud-saasaccelerator-management-logs-v1",
                    ))]
                    pub mod v1 {
                        #[cfg(any(
                            feature = "google-cloud-saasaccelerator-management-logs-v1",
                        ))]
                        include!("bytes_btree_map/google.cloud.saasaccelerator.management.logs.v1.rs");
                    }
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-visionai-v1",
            feature = "google-cloud-visionai-v1alpha1",
        ))]
        pub mod visionai {
            #[cfg(any(
                feature = "google-cloud-visionai-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-visionai-v1",
                ))]
                include!("bytes_btree_map/google.cloud.visionai.v1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-visionai-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-cloud-visionai-v1alpha1",
                ))]
                include!("bytes_btree_map/google.cloud.visionai.v1alpha1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-recommendationengine-v1beta1",
        ))]
        pub mod recommendationengine {
            #[cfg(any(
                feature = "google-cloud-recommendationengine-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-recommendationengine-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.recommendationengine.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-datafusion-v1",
            feature = "google-cloud-datafusion-v1beta1",
        ))]
        pub mod datafusion {
            #[cfg(any(
                feature = "google-cloud-datafusion-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-datafusion-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.datafusion.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-datafusion-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-datafusion-v1",
                ))]
                include!("bytes_btree_map/google.cloud.datafusion.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-filestore-v1",
            feature = "google-cloud-filestore-v1beta1",
        ))]
        pub mod filestore {
            #[cfg(any(
                feature = "google-cloud-filestore-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-filestore-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.filestore.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-cloud-filestore-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-filestore-v1",
                ))]
                include!("bytes_btree_map/google.cloud.filestore.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-securitycentermanagement-v1",
        ))]
        pub mod securitycentermanagement {
            #[cfg(any(
                feature = "google-cloud-securitycentermanagement-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-securitycentermanagement-v1",
                ))]
                include!("bytes_btree_map/google.cloud.securitycentermanagement.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-phishingprotection-v1beta1",
        ))]
        pub mod phishingprotection {
            #[cfg(any(
                feature = "google-cloud-phishingprotection-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-phishingprotection-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.phishingprotection.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-migrationcenter-v1",
        ))]
        pub mod migrationcenter {
            #[cfg(any(
                feature = "google-cloud-migrationcenter-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-migrationcenter-v1",
                ))]
                include!("bytes_btree_map/google.cloud.migrationcenter.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-storageinsights-v1",
        ))]
        pub mod storageinsights {
            #[cfg(any(
                feature = "google-cloud-storageinsights-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-storageinsights-v1",
                ))]
                include!("bytes_btree_map/google.cloud.storageinsights.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-abuseevent-logging-v1",
        ))]
        pub mod abuseevent {
            #[cfg(any(
                feature = "google-cloud-abuseevent-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-cloud-abuseevent-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-cloud-abuseevent-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.cloud.abuseevent.logging.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-cloud-deploy-v1",
        ))]
        pub mod deploy {
            #[cfg(any(
                feature = "google-cloud-deploy-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-deploy-v1",
                ))]
                include!("bytes_btree_map/google.cloud.deploy.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-edgecontainer-v1",
        ))]
        pub mod edgecontainer {
            #[cfg(any(
                feature = "google-cloud-edgecontainer-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-edgecontainer-v1",
                ))]
                include!("bytes_btree_map/google.cloud.edgecontainer.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-optimization-v1",
        ))]
        pub mod optimization {
            #[cfg(any(
                feature = "google-cloud-optimization-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-optimization-v1",
                ))]
                include!("bytes_btree_map/google.cloud.optimization.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-managedkafka-v1",
        ))]
        pub mod managedkafka {
            #[cfg(any(
                feature = "google-cloud-managedkafka-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-managedkafka-v1",
                ))]
                include!("bytes_btree_map/google.cloud.managedkafka.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-iot-v1",
        ))]
        pub mod iot {
            #[cfg(any(
                feature = "google-cloud-iot-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-iot-v1",
                ))]
                include!("bytes_btree_map/google.cloud.iot.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-securesourcemanager-v1",
        ))]
        pub mod securesourcemanager {
            #[cfg(any(
                feature = "google-cloud-securesourcemanager-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-securesourcemanager-v1",
                ))]
                include!("bytes_btree_map/google.cloud.securesourcemanager.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-datalabeling-v1beta1",
        ))]
        pub mod datalabeling {
            #[cfg(any(
                feature = "google-cloud-datalabeling-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-cloud-datalabeling-v1beta1",
                ))]
                include!("bytes_btree_map/google.cloud.datalabeling.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-cloud-pubsublite-v1",
        ))]
        pub mod pubsublite {
            #[cfg(any(
                feature = "google-cloud-pubsublite-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-cloud-pubsublite-v1",
                ))]
                include!("bytes_btree_map/google.cloud.pubsublite.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-longrunning",
    ))]
    pub mod longrunning {
        #[cfg(any(
            feature = "google-longrunning",
        ))]
        include!("bytes_btree_map/google.longrunning.rs");
    }
    #[cfg(any(
        feature = "google-shopping-css-v1",
        feature = "google-shopping-merchant-accounts-v1beta",
        feature = "google-shopping-merchant-conversions-v1beta",
        feature = "google-shopping-merchant-datasources-v1beta",
        feature = "google-shopping-merchant-inventories-v1beta",
        feature = "google-shopping-merchant-lfp-v1beta",
        feature = "google-shopping-merchant-notifications-v1beta",
        feature = "google-shopping-merchant-products-v1beta",
        feature = "google-shopping-merchant-promotions-v1beta",
        feature = "google-shopping-merchant-quota-v1beta",
        feature = "google-shopping-merchant-reports-v1beta",
        feature = "google-shopping-type",
    ))]
    pub mod shopping {
        #[cfg(any(
            feature = "google-shopping-merchant-accounts-v1beta",
            feature = "google-shopping-merchant-conversions-v1beta",
            feature = "google-shopping-merchant-datasources-v1beta",
            feature = "google-shopping-merchant-inventories-v1beta",
            feature = "google-shopping-merchant-lfp-v1beta",
            feature = "google-shopping-merchant-notifications-v1beta",
            feature = "google-shopping-merchant-products-v1beta",
            feature = "google-shopping-merchant-promotions-v1beta",
            feature = "google-shopping-merchant-quota-v1beta",
            feature = "google-shopping-merchant-reports-v1beta",
        ))]
        pub mod merchant {
            #[cfg(any(
                feature = "google-shopping-merchant-datasources-v1beta",
            ))]
            pub mod datasources {
                #[cfg(any(
                    feature = "google-shopping-merchant-datasources-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-datasources-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.datasources.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-products-v1beta",
            ))]
            pub mod products {
                #[cfg(any(
                    feature = "google-shopping-merchant-products-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-products-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.products.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-inventories-v1beta",
            ))]
            pub mod inventories {
                #[cfg(any(
                    feature = "google-shopping-merchant-inventories-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-inventories-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.inventories.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-quota-v1beta",
            ))]
            pub mod quota {
                #[cfg(any(
                    feature = "google-shopping-merchant-quota-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-quota-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.quota.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-conversions-v1beta",
            ))]
            pub mod conversions {
                #[cfg(any(
                    feature = "google-shopping-merchant-conversions-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-conversions-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.conversions.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-promotions-v1beta",
            ))]
            pub mod promotions {
                #[cfg(any(
                    feature = "google-shopping-merchant-promotions-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-promotions-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.promotions.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-reports-v1beta",
            ))]
            pub mod reports {
                #[cfg(any(
                    feature = "google-shopping-merchant-reports-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-reports-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.reports.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-lfp-v1beta",
            ))]
            pub mod lfp {
                #[cfg(any(
                    feature = "google-shopping-merchant-lfp-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-lfp-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.lfp.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-notifications-v1beta",
            ))]
            pub mod notifications {
                #[cfg(any(
                    feature = "google-shopping-merchant-notifications-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-notifications-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.notifications.v1beta.rs");
                }
            }
            #[cfg(any(
                feature = "google-shopping-merchant-accounts-v1beta",
            ))]
            pub mod accounts {
                #[cfg(any(
                    feature = "google-shopping-merchant-accounts-v1beta",
                ))]
                pub mod v1beta {
                    #[cfg(any(
                        feature = "google-shopping-merchant-accounts-v1beta",
                    ))]
                    include!("bytes_btree_map/google.shopping.merchant.accounts.v1beta.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-shopping-type",
        ))]
        pub mod r#type {
            #[cfg(any(
                feature = "google-shopping-type",
            ))]
            include!("bytes_btree_map/google.shopping.r#type.rs");
        }
        #[cfg(any(
            feature = "google-shopping-css-v1",
        ))]
        pub mod css {
            #[cfg(any(
                feature = "google-shopping-css-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-shopping-css-v1",
                ))]
                include!("bytes_btree_map/google.shopping.css.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-ai-generativelanguage-v1",
        feature = "google-ai-generativelanguage-v1beta",
        feature = "google-ai-generativelanguage-v1beta2",
        feature = "google-ai-generativelanguage-v1beta3",
    ))]
    pub mod ai {
        #[cfg(any(
            feature = "google-ai-generativelanguage-v1",
            feature = "google-ai-generativelanguage-v1beta",
            feature = "google-ai-generativelanguage-v1beta2",
            feature = "google-ai-generativelanguage-v1beta3",
        ))]
        pub mod generativelanguage {
            #[cfg(any(
                feature = "google-ai-generativelanguage-v1beta3",
            ))]
            pub mod v1beta3 {
                #[cfg(any(
                    feature = "google-ai-generativelanguage-v1beta3",
                ))]
                include!("bytes_btree_map/google.ai.generativelanguage.v1beta3.rs");
            }
            #[cfg(any(
                feature = "google-ai-generativelanguage-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-ai-generativelanguage-v1beta2",
                ))]
                include!("bytes_btree_map/google.ai.generativelanguage.v1beta2.rs");
            }
            #[cfg(any(
                feature = "google-ai-generativelanguage-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-ai-generativelanguage-v1beta",
                ))]
                include!("bytes_btree_map/google.ai.generativelanguage.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-ai-generativelanguage-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-ai-generativelanguage-v1",
                ))]
                include!("bytes_btree_map/google.ai.generativelanguage.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-home-enterprise-sdm-v1",
        feature = "google-home-graph-v1",
    ))]
    pub mod home {
        #[cfg(any(
            feature = "google-home-graph-v1",
        ))]
        pub mod graph {
            #[cfg(any(
                feature = "google-home-graph-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-home-graph-v1",
                ))]
                include!("bytes_btree_map/google.home.graph.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-home-enterprise-sdm-v1",
        ))]
        pub mod enterprise {
            #[cfg(any(
                feature = "google-home-enterprise-sdm-v1",
            ))]
            pub mod sdm {
                #[cfg(any(
                    feature = "google-home-enterprise-sdm-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-home-enterprise-sdm-v1",
                    ))]
                    include!("bytes_btree_map/google.home.enterprise.sdm.v1.rs");
                }
            }
        }
    }
    #[cfg(any(
        feature = "google-ads-admanager-v1",
        feature = "google-ads-admob-v1",
        feature = "google-ads-googleads-v15-common",
        feature = "google-ads-googleads-v15-enums",
        feature = "google-ads-googleads-v15-errors",
        feature = "google-ads-googleads-v15-resources",
        feature = "google-ads-googleads-v15-services",
        feature = "google-ads-googleads-v16-common",
        feature = "google-ads-googleads-v16-enums",
        feature = "google-ads-googleads-v16-errors",
        feature = "google-ads-googleads-v16-resources",
        feature = "google-ads-googleads-v16-services",
        feature = "google-ads-googleads-v17-common",
        feature = "google-ads-googleads-v17-enums",
        feature = "google-ads-googleads-v17-errors",
        feature = "google-ads-googleads-v17-resources",
        feature = "google-ads-googleads-v17-services",
        feature = "google-ads-searchads360-v0-common",
        feature = "google-ads-searchads360-v0-enums",
        feature = "google-ads-searchads360-v0-errors",
        feature = "google-ads-searchads360-v0-resources",
        feature = "google-ads-searchads360-v0-services",
    ))]
    pub mod ads {
        #[cfg(any(
            feature = "google-ads-searchads360-v0-common",
            feature = "google-ads-searchads360-v0-enums",
            feature = "google-ads-searchads360-v0-errors",
            feature = "google-ads-searchads360-v0-resources",
            feature = "google-ads-searchads360-v0-services",
        ))]
        pub mod searchads360 {
            #[cfg(any(
                feature = "google-ads-searchads360-v0-common",
                feature = "google-ads-searchads360-v0-enums",
                feature = "google-ads-searchads360-v0-errors",
                feature = "google-ads-searchads360-v0-resources",
                feature = "google-ads-searchads360-v0-services",
            ))]
            pub mod v0 {
                #[cfg(any(
                    feature = "google-ads-searchads360-v0-services",
                ))]
                pub mod services {
                    #[cfg(any(
                        feature = "google-ads-searchads360-v0-services",
                    ))]
                    include!("bytes_btree_map/google.ads.searchads360.v0.services.rs");
                }
                #[cfg(any(
                    feature = "google-ads-searchads360-v0-enums",
                ))]
                pub mod enums {
                    #[cfg(any(
                        feature = "google-ads-searchads360-v0-enums",
                    ))]
                    include!("bytes_btree_map/google.ads.searchads360.v0.enums.rs");
                }
                #[cfg(any(
                    feature = "google-ads-searchads360-v0-common",
                ))]
                pub mod common {
                    #[cfg(any(
                        feature = "google-ads-searchads360-v0-common",
                    ))]
                    include!("bytes_btree_map/google.ads.searchads360.v0.common.rs");
                }
                #[cfg(any(
                    feature = "google-ads-searchads360-v0-resources",
                ))]
                pub mod resources {
                    #[cfg(any(
                        feature = "google-ads-searchads360-v0-resources",
                    ))]
                    include!("bytes_btree_map/google.ads.searchads360.v0.resources.rs");
                }
                #[cfg(any(
                    feature = "google-ads-searchads360-v0-errors",
                ))]
                pub mod errors {
                    #[cfg(any(
                        feature = "google-ads-searchads360-v0-errors",
                    ))]
                    include!("bytes_btree_map/google.ads.searchads360.v0.errors.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-ads-googleads-v15-common",
            feature = "google-ads-googleads-v15-enums",
            feature = "google-ads-googleads-v15-errors",
            feature = "google-ads-googleads-v15-resources",
            feature = "google-ads-googleads-v15-services",
            feature = "google-ads-googleads-v16-common",
            feature = "google-ads-googleads-v16-enums",
            feature = "google-ads-googleads-v16-errors",
            feature = "google-ads-googleads-v16-resources",
            feature = "google-ads-googleads-v16-services",
            feature = "google-ads-googleads-v17-common",
            feature = "google-ads-googleads-v17-enums",
            feature = "google-ads-googleads-v17-errors",
            feature = "google-ads-googleads-v17-resources",
            feature = "google-ads-googleads-v17-services",
        ))]
        pub mod googleads {
            #[cfg(any(
                feature = "google-ads-googleads-v16-common",
                feature = "google-ads-googleads-v16-enums",
                feature = "google-ads-googleads-v16-errors",
                feature = "google-ads-googleads-v16-resources",
                feature = "google-ads-googleads-v16-services",
            ))]
            pub mod v16 {
                #[cfg(any(
                    feature = "google-ads-googleads-v16-enums",
                ))]
                pub mod enums {
                    #[cfg(any(
                        feature = "google-ads-googleads-v16-enums",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v16.enums.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v16-services",
                ))]
                pub mod services {
                    #[cfg(any(
                        feature = "google-ads-googleads-v16-services",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v16.services.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v16-resources",
                ))]
                pub mod resources {
                    #[cfg(any(
                        feature = "google-ads-googleads-v16-resources",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v16.resources.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v16-common",
                ))]
                pub mod common {
                    #[cfg(any(
                        feature = "google-ads-googleads-v16-common",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v16.common.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v16-errors",
                ))]
                pub mod errors {
                    #[cfg(any(
                        feature = "google-ads-googleads-v16-errors",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v16.errors.rs");
                }
            }
            #[cfg(any(
                feature = "google-ads-googleads-v17-common",
                feature = "google-ads-googleads-v17-enums",
                feature = "google-ads-googleads-v17-errors",
                feature = "google-ads-googleads-v17-resources",
                feature = "google-ads-googleads-v17-services",
            ))]
            pub mod v17 {
                #[cfg(any(
                    feature = "google-ads-googleads-v17-resources",
                ))]
                pub mod resources {
                    #[cfg(any(
                        feature = "google-ads-googleads-v17-resources",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v17.resources.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v17-errors",
                ))]
                pub mod errors {
                    #[cfg(any(
                        feature = "google-ads-googleads-v17-errors",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v17.errors.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v17-services",
                ))]
                pub mod services {
                    #[cfg(any(
                        feature = "google-ads-googleads-v17-services",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v17.services.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v17-common",
                ))]
                pub mod common {
                    #[cfg(any(
                        feature = "google-ads-googleads-v17-common",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v17.common.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v17-enums",
                ))]
                pub mod enums {
                    #[cfg(any(
                        feature = "google-ads-googleads-v17-enums",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v17.enums.rs");
                }
            }
            #[cfg(any(
                feature = "google-ads-googleads-v15-common",
                feature = "google-ads-googleads-v15-enums",
                feature = "google-ads-googleads-v15-errors",
                feature = "google-ads-googleads-v15-resources",
                feature = "google-ads-googleads-v15-services",
            ))]
            pub mod v15 {
                #[cfg(any(
                    feature = "google-ads-googleads-v15-errors",
                ))]
                pub mod errors {
                    #[cfg(any(
                        feature = "google-ads-googleads-v15-errors",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v15.errors.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v15-resources",
                ))]
                pub mod resources {
                    #[cfg(any(
                        feature = "google-ads-googleads-v15-resources",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v15.resources.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v15-services",
                ))]
                pub mod services {
                    #[cfg(any(
                        feature = "google-ads-googleads-v15-services",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v15.services.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v15-enums",
                ))]
                pub mod enums {
                    #[cfg(any(
                        feature = "google-ads-googleads-v15-enums",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v15.enums.rs");
                }
                #[cfg(any(
                    feature = "google-ads-googleads-v15-common",
                ))]
                pub mod common {
                    #[cfg(any(
                        feature = "google-ads-googleads-v15-common",
                    ))]
                    include!("bytes_btree_map/google.ads.googleads.v15.common.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-ads-admob-v1",
        ))]
        pub mod admob {
            #[cfg(any(
                feature = "google-ads-admob-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-ads-admob-v1",
                ))]
                include!("bytes_btree_map/google.ads.admob.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-ads-admanager-v1",
        ))]
        pub mod admanager {
            #[cfg(any(
                feature = "google-ads-admanager-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-ads-admanager-v1",
                ))]
                include!("bytes_btree_map/google.ads.admanager.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-devtools-artifactregistry-v1",
        feature = "google-devtools-artifactregistry-v1beta2",
        feature = "google-devtools-build-v1",
        feature = "google-devtools-cloudbuild-v1",
        feature = "google-devtools-cloudbuild-v2",
        feature = "google-devtools-clouddebugger-v2",
        feature = "google-devtools-clouderrorreporting-v1beta1",
        feature = "google-devtools-cloudprofiler-v2",
        feature = "google-devtools-cloudtrace-v1",
        feature = "google-devtools-cloudtrace-v2",
        feature = "google-devtools-containeranalysis-v1",
        feature = "google-devtools-containeranalysis-v1beta1",
        feature = "google-devtools-remoteworkers-v1test2",
        feature = "google-devtools-resultstore-v2",
        feature = "google-devtools-source-v1",
        feature = "google-devtools-sourcerepo-v1",
        feature = "google-devtools-testing-v1",
    ))]
    pub mod devtools {
        #[cfg(any(
            feature = "google-devtools-sourcerepo-v1",
        ))]
        pub mod sourcerepo {
            #[cfg(any(
                feature = "google-devtools-sourcerepo-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-sourcerepo-v1",
                ))]
                include!("bytes_btree_map/google.devtools.sourcerepo.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-cloudbuild-v1",
            feature = "google-devtools-cloudbuild-v2",
        ))]
        pub mod cloudbuild {
            #[cfg(any(
                feature = "google-devtools-cloudbuild-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-devtools-cloudbuild-v2",
                ))]
                include!("bytes_btree_map/google.devtools.cloudbuild.v2.rs");
            }
            #[cfg(any(
                feature = "google-devtools-cloudbuild-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-cloudbuild-v1",
                ))]
                include!("bytes_btree_map/google.devtools.cloudbuild.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-containeranalysis-v1",
            feature = "google-devtools-containeranalysis-v1beta1",
        ))]
        pub mod containeranalysis {
            #[cfg(any(
                feature = "google-devtools-containeranalysis-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-containeranalysis-v1",
                ))]
                include!("bytes_btree_map/google.devtools.containeranalysis.v1.rs");
            }
            #[cfg(any(
                feature = "google-devtools-containeranalysis-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-devtools-containeranalysis-v1beta1",
                ))]
                include!("bytes_btree_map/google.devtools.containeranalysis.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-source-v1",
        ))]
        pub mod source {
            #[cfg(any(
                feature = "google-devtools-source-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-source-v1",
                ))]
                include!("bytes_btree_map/google.devtools.source.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-artifactregistry-v1",
            feature = "google-devtools-artifactregistry-v1beta2",
        ))]
        pub mod artifactregistry {
            #[cfg(any(
                feature = "google-devtools-artifactregistry-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-artifactregistry-v1",
                ))]
                include!("bytes_btree_map/google.devtools.artifactregistry.v1.rs");
            }
            #[cfg(any(
                feature = "google-devtools-artifactregistry-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-devtools-artifactregistry-v1beta2",
                ))]
                include!("bytes_btree_map/google.devtools.artifactregistry.v1beta2.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-build-v1",
        ))]
        pub mod build {
            #[cfg(any(
                feature = "google-devtools-build-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-build-v1",
                ))]
                include!("bytes_btree_map/google.devtools.build.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-resultstore-v2",
        ))]
        pub mod resultstore {
            #[cfg(any(
                feature = "google-devtools-resultstore-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-devtools-resultstore-v2",
                ))]
                include!("bytes_btree_map/google.devtools.resultstore.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-cloudprofiler-v2",
        ))]
        pub mod cloudprofiler {
            #[cfg(any(
                feature = "google-devtools-cloudprofiler-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-devtools-cloudprofiler-v2",
                ))]
                include!("bytes_btree_map/google.devtools.cloudprofiler.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-clouderrorreporting-v1beta1",
        ))]
        pub mod clouderrorreporting {
            #[cfg(any(
                feature = "google-devtools-clouderrorreporting-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-devtools-clouderrorreporting-v1beta1",
                ))]
                include!("bytes_btree_map/google.devtools.clouderrorreporting.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-testing-v1",
        ))]
        pub mod testing {
            #[cfg(any(
                feature = "google-devtools-testing-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-testing-v1",
                ))]
                include!("bytes_btree_map/google.devtools.testing.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-cloudtrace-v1",
            feature = "google-devtools-cloudtrace-v2",
        ))]
        pub mod cloudtrace {
            #[cfg(any(
                feature = "google-devtools-cloudtrace-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-devtools-cloudtrace-v1",
                ))]
                include!("bytes_btree_map/google.devtools.cloudtrace.v1.rs");
            }
            #[cfg(any(
                feature = "google-devtools-cloudtrace-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-devtools-cloudtrace-v2",
                ))]
                include!("bytes_btree_map/google.devtools.cloudtrace.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-remoteworkers-v1test2",
        ))]
        pub mod remoteworkers {
            #[cfg(any(
                feature = "google-devtools-remoteworkers-v1test2",
            ))]
            pub mod v1test2 {
                #[cfg(any(
                    feature = "google-devtools-remoteworkers-v1test2",
                ))]
                include!("bytes_btree_map/google.devtools.remoteworkers.v1test2.rs");
            }
        }
        #[cfg(any(
            feature = "google-devtools-clouddebugger-v2",
        ))]
        pub mod clouddebugger {
            #[cfg(any(
                feature = "google-devtools-clouddebugger-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-devtools-clouddebugger-v2",
                ))]
                include!("bytes_btree_map/google.devtools.clouddebugger.v2.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-compute-logging-dr-v1",
        feature = "google-compute-logging-gdnsusage-v1",
    ))]
    pub mod compute {
        #[cfg(any(
            feature = "google-compute-logging-dr-v1",
            feature = "google-compute-logging-gdnsusage-v1",
        ))]
        pub mod logging {
            #[cfg(any(
                feature = "google-compute-logging-dr-v1",
            ))]
            pub mod dr {
                #[cfg(any(
                    feature = "google-compute-logging-dr-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-compute-logging-dr-v1",
                    ))]
                    include!("bytes_btree_map/google.compute.logging.dr.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-compute-logging-gdnsusage-v1",
            ))]
            pub mod gdnsusage {
                #[cfg(any(
                    feature = "google-compute-logging-gdnsusage-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-compute-logging-gdnsusage-v1",
                    ))]
                    include!("bytes_btree_map/google.compute.logging.gdnsusage.v1.rs");
                }
            }
        }
    }
    #[cfg(any(
        feature = "google-maps-addressvalidation-v1",
        feature = "google-maps-aerialview-v1",
        feature = "google-maps-areainsights-v1",
        feature = "google-maps-mapsplatformdatasets-v1",
        feature = "google-maps-mobilitybilling-logs-v1",
        feature = "google-maps-places-v1",
        feature = "google-maps-playablelocations-v3",
        feature = "google-maps-playablelocations-v3-sample",
        feature = "google-maps-regionlookup-v1alpha",
        feature = "google-maps-roads-v1op",
        feature = "google-maps-routeoptimization-v1",
        feature = "google-maps-routes-v1",
        feature = "google-maps-routes-v1alpha",
        feature = "google-maps-routing-v2",
        feature = "google-maps-solar-v1",
        feature = "google-maps-unity",
    ))]
    pub mod maps {
        #[cfg(any(
            feature = "google-maps-mapsplatformdatasets-v1",
        ))]
        pub mod mapsplatformdatasets {
            #[cfg(any(
                feature = "google-maps-mapsplatformdatasets-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-mapsplatformdatasets-v1",
                ))]
                include!("bytes_btree_map/google.maps.mapsplatformdatasets.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-playablelocations-v3",
            feature = "google-maps-playablelocations-v3-sample",
        ))]
        pub mod playablelocations {
            #[cfg(any(
                feature = "google-maps-playablelocations-v3",
                feature = "google-maps-playablelocations-v3-sample",
            ))]
            pub mod v3 {
                #[cfg(any(
                    feature = "google-maps-playablelocations-v3",
                ))]
                include!("bytes_btree_map/google.maps.playablelocations.v3.rs");
                #[cfg(any(
                    feature = "google-maps-playablelocations-v3-sample",
                ))]
                pub mod sample {
                    #[cfg(any(
                        feature = "google-maps-playablelocations-v3-sample",
                    ))]
                    include!("bytes_btree_map/google.maps.playablelocations.v3.sample.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-maps-regionlookup-v1alpha",
        ))]
        pub mod regionlookup {
            #[cfg(any(
                feature = "google-maps-regionlookup-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-maps-regionlookup-v1alpha",
                ))]
                include!("bytes_btree_map/google.maps.regionlookup.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-areainsights-v1",
        ))]
        pub mod areainsights {
            #[cfg(any(
                feature = "google-maps-areainsights-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-areainsights-v1",
                ))]
                include!("bytes_btree_map/google.maps.areainsights.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-aerialview-v1",
        ))]
        pub mod aerialview {
            #[cfg(any(
                feature = "google-maps-aerialview-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-aerialview-v1",
                ))]
                include!("bytes_btree_map/google.maps.aerialview.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-routes-v1",
            feature = "google-maps-routes-v1alpha",
        ))]
        pub mod routes {
            #[cfg(any(
                feature = "google-maps-routes-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-routes-v1",
                ))]
                include!("bytes_btree_map/google.maps.routes.v1.rs");
            }
            #[cfg(any(
                feature = "google-maps-routes-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-maps-routes-v1alpha",
                ))]
                include!("bytes_btree_map/google.maps.routes.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-addressvalidation-v1",
        ))]
        pub mod addressvalidation {
            #[cfg(any(
                feature = "google-maps-addressvalidation-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-addressvalidation-v1",
                ))]
                include!("bytes_btree_map/google.maps.addressvalidation.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-solar-v1",
        ))]
        pub mod solar {
            #[cfg(any(
                feature = "google-maps-solar-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-solar-v1",
                ))]
                include!("bytes_btree_map/google.maps.solar.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-mobilitybilling-logs-v1",
        ))]
        pub mod mobilitybilling {
            #[cfg(any(
                feature = "google-maps-mobilitybilling-logs-v1",
            ))]
            pub mod logs {
                #[cfg(any(
                    feature = "google-maps-mobilitybilling-logs-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-maps-mobilitybilling-logs-v1",
                    ))]
                    include!("bytes_btree_map/google.maps.mobilitybilling.logs.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-maps-unity",
        ))]
        pub mod unity {
            #[cfg(any(
                feature = "google-maps-unity",
            ))]
            include!("bytes_btree_map/google.maps.unity.rs");
        }
        #[cfg(any(
            feature = "google-maps-routeoptimization-v1",
        ))]
        pub mod routeoptimization {
            #[cfg(any(
                feature = "google-maps-routeoptimization-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-routeoptimization-v1",
                ))]
                include!("bytes_btree_map/google.maps.routeoptimization.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-roads-v1op",
        ))]
        pub mod roads {
            #[cfg(any(
                feature = "google-maps-roads-v1op",
            ))]
            pub mod v1op {
                #[cfg(any(
                    feature = "google-maps-roads-v1op",
                ))]
                include!("bytes_btree_map/google.maps.roads.v1op.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-places-v1",
        ))]
        pub mod places {
            #[cfg(any(
                feature = "google-maps-places-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-maps-places-v1",
                ))]
                include!("bytes_btree_map/google.maps.places.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-maps-routing-v2",
        ))]
        pub mod routing {
            #[cfg(any(
                feature = "google-maps-routing-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-maps-routing-v2",
                ))]
                include!("bytes_btree_map/google.maps.routing.v2.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-actions-sdk-v2",
        feature = "google-actions-sdk-v2-conversation",
        feature = "google-actions-sdk-v2-interactionmodel",
        feature = "google-actions-sdk-v2-interactionmodel-prompt",
        feature = "google-actions-sdk-v2-interactionmodel-type",
        feature = "google-actions-type",
    ))]
    pub mod actions {
        #[cfg(any(
            feature = "google-actions-sdk-v2",
            feature = "google-actions-sdk-v2-conversation",
            feature = "google-actions-sdk-v2-interactionmodel",
            feature = "google-actions-sdk-v2-interactionmodel-prompt",
            feature = "google-actions-sdk-v2-interactionmodel-type",
        ))]
        pub mod sdk {
            #[cfg(any(
                feature = "google-actions-sdk-v2",
                feature = "google-actions-sdk-v2-conversation",
                feature = "google-actions-sdk-v2-interactionmodel",
                feature = "google-actions-sdk-v2-interactionmodel-prompt",
                feature = "google-actions-sdk-v2-interactionmodel-type",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-actions-sdk-v2",
                ))]
                include!("bytes_btree_map/google.actions.sdk.v2.rs");
                #[cfg(any(
                    feature = "google-actions-sdk-v2-interactionmodel",
                    feature = "google-actions-sdk-v2-interactionmodel-prompt",
                    feature = "google-actions-sdk-v2-interactionmodel-type",
                ))]
                pub mod interactionmodel {
                    #[cfg(any(
                        feature = "google-actions-sdk-v2-interactionmodel",
                    ))]
                    include!("bytes_btree_map/google.actions.sdk.v2.interactionmodel.rs");
                    #[cfg(any(
                        feature = "google-actions-sdk-v2-interactionmodel-type",
                    ))]
                    pub mod r#type {
                        #[cfg(any(
                            feature = "google-actions-sdk-v2-interactionmodel-type",
                        ))]
                        include!("bytes_btree_map/google.actions.sdk.v2.interactionmodel.r#type.rs");
                    }
                    #[cfg(any(
                        feature = "google-actions-sdk-v2-interactionmodel-prompt",
                    ))]
                    pub mod prompt {
                        #[cfg(any(
                            feature = "google-actions-sdk-v2-interactionmodel-prompt",
                        ))]
                        include!("bytes_btree_map/google.actions.sdk.v2.interactionmodel.prompt.rs");
                    }
                }
                #[cfg(any(
                    feature = "google-actions-sdk-v2-conversation",
                ))]
                pub mod conversation {
                    #[cfg(any(
                        feature = "google-actions-sdk-v2-conversation",
                    ))]
                    include!("bytes_btree_map/google.actions.sdk.v2.conversation.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-actions-type",
        ))]
        pub mod r#type {
            #[cfg(any(
                feature = "google-actions-type",
            ))]
            include!("bytes_btree_map/google.actions.r#type.rs");
        }
    }
    #[cfg(any(
        feature = "google-logging-type",
        feature = "google-logging-v2",
    ))]
    pub mod logging {
        #[cfg(any(
            feature = "google-logging-v2",
        ))]
        pub mod v2 {
            #[cfg(any(
                feature = "google-logging-v2",
            ))]
            include!("bytes_btree_map/google.logging.v2.rs");
        }
        #[cfg(any(
            feature = "google-logging-type",
        ))]
        pub mod r#type {
            #[cfg(any(
                feature = "google-logging-type",
            ))]
            include!("bytes_btree_map/google.logging.r#type.rs");
        }
    }
    #[cfg(any(
        feature = "google-datastore-admin-v1",
        feature = "google-datastore-admin-v1beta1",
        feature = "google-datastore-v1",
        feature = "google-datastore-v1beta3",
    ))]
    pub mod datastore {
        #[cfg(any(
            feature = "google-datastore-admin-v1",
            feature = "google-datastore-admin-v1beta1",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-datastore-admin-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-datastore-admin-v1",
                ))]
                include!("bytes_btree_map/google.datastore.admin.v1.rs");
            }
            #[cfg(any(
                feature = "google-datastore-admin-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-datastore-admin-v1beta1",
                ))]
                include!("bytes_btree_map/google.datastore.admin.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-datastore-v1beta3",
        ))]
        pub mod v1beta3 {
            #[cfg(any(
                feature = "google-datastore-v1beta3",
            ))]
            include!("bytes_btree_map/google.datastore.v1beta3.rs");
        }
        #[cfg(any(
            feature = "google-datastore-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-datastore-v1",
            ))]
            include!("bytes_btree_map/google.datastore.v1.rs");
        }
    }
    #[cfg(any(
        feature = "google-spanner-admin-database-v1",
        feature = "google-spanner-admin-instance-v1",
        feature = "google-spanner-executor-v1",
        feature = "google-spanner-v1",
    ))]
    pub mod spanner {
        #[cfg(any(
            feature = "google-spanner-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-spanner-v1",
            ))]
            include!("bytes_btree_map/google.spanner.v1.rs");
        }
        #[cfg(any(
            feature = "google-spanner-admin-database-v1",
            feature = "google-spanner-admin-instance-v1",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-spanner-admin-instance-v1",
            ))]
            pub mod instance {
                #[cfg(any(
                    feature = "google-spanner-admin-instance-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-spanner-admin-instance-v1",
                    ))]
                    include!("bytes_btree_map/google.spanner.admin.instance.v1.rs");
                }
            }
            #[cfg(any(
                feature = "google-spanner-admin-database-v1",
            ))]
            pub mod database {
                #[cfg(any(
                    feature = "google-spanner-admin-database-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-spanner-admin-database-v1",
                    ))]
                    include!("bytes_btree_map/google.spanner.admin.database.v1.rs");
                }
            }
        }
        #[cfg(any(
            feature = "google-spanner-executor-v1",
        ))]
        pub mod executor {
            #[cfg(any(
                feature = "google-spanner-executor-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-spanner-executor-v1",
                ))]
                include!("bytes_btree_map/google.spanner.executor.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-type",
    ))]
    pub mod r#type {
        #[cfg(any(
            feature = "google-type",
        ))]
        include!("bytes_btree_map/google.r#type.rs");
    }
    #[cfg(any(
        feature = "google-genomics-v1",
        feature = "google-genomics-v1alpha2",
    ))]
    pub mod genomics {
        #[cfg(any(
            feature = "google-genomics-v1alpha2",
        ))]
        pub mod v1alpha2 {
            #[cfg(any(
                feature = "google-genomics-v1alpha2",
            ))]
            include!("bytes_btree_map/google.genomics.v1alpha2.rs");
        }
        #[cfg(any(
            feature = "google-genomics-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-genomics-v1",
            ))]
            include!("bytes_btree_map/google.genomics.v1.rs");
        }
    }
    #[cfg(any(
        feature = "google-api",
        feature = "google-api-apikeys-v2",
        feature = "google-api-cloudquotas-v1",
        feature = "google-api-expr-conformance-v1alpha1",
        feature = "google-api-expr-v1alpha1",
        feature = "google-api-expr-v1beta1",
        feature = "google-api-servicecontrol-v1",
        feature = "google-api-servicecontrol-v2",
        feature = "google-api-servicemanagement-v1",
        feature = "google-api-serviceusage-v1",
        feature = "google-api-serviceusage-v1beta1",
    ))]
    pub mod api {
        #[cfg(any(
            feature = "google-api",
        ))]
        include!("bytes_btree_map/google.api.rs");
        #[cfg(any(
            feature = "google-api-expr-conformance-v1alpha1",
            feature = "google-api-expr-v1alpha1",
            feature = "google-api-expr-v1beta1",
        ))]
        pub mod expr {
            #[cfg(any(
                feature = "google-api-expr-conformance-v1alpha1",
            ))]
            pub mod conformance {
                #[cfg(any(
                    feature = "google-api-expr-conformance-v1alpha1",
                ))]
                pub mod v1alpha1 {
                    #[cfg(any(
                        feature = "google-api-expr-conformance-v1alpha1",
                    ))]
                    include!("bytes_btree_map/google.api.expr.conformance.v1alpha1.rs");
                }
            }
            #[cfg(any(
                feature = "google-api-expr-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-api-expr-v1beta1",
                ))]
                include!("bytes_btree_map/google.api.expr.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-api-expr-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-api-expr-v1alpha1",
                ))]
                include!("bytes_btree_map/google.api.expr.v1alpha1.rs");
            }
        }
        #[cfg(any(
            feature = "google-api-servicecontrol-v1",
            feature = "google-api-servicecontrol-v2",
        ))]
        pub mod servicecontrol {
            #[cfg(any(
                feature = "google-api-servicecontrol-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-api-servicecontrol-v1",
                ))]
                include!("bytes_btree_map/google.api.servicecontrol.v1.rs");
            }
            #[cfg(any(
                feature = "google-api-servicecontrol-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-api-servicecontrol-v2",
                ))]
                include!("bytes_btree_map/google.api.servicecontrol.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-api-servicemanagement-v1",
        ))]
        pub mod servicemanagement {
            #[cfg(any(
                feature = "google-api-servicemanagement-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-api-servicemanagement-v1",
                ))]
                include!("bytes_btree_map/google.api.servicemanagement.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-api-serviceusage-v1",
            feature = "google-api-serviceusage-v1beta1",
        ))]
        pub mod serviceusage {
            #[cfg(any(
                feature = "google-api-serviceusage-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-api-serviceusage-v1beta1",
                ))]
                include!("bytes_btree_map/google.api.serviceusage.v1beta1.rs");
            }
            #[cfg(any(
                feature = "google-api-serviceusage-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-api-serviceusage-v1",
                ))]
                include!("bytes_btree_map/google.api.serviceusage.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-api-apikeys-v2",
        ))]
        pub mod apikeys {
            #[cfg(any(
                feature = "google-api-apikeys-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-api-apikeys-v2",
                ))]
                include!("bytes_btree_map/google.api.apikeys.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-api-cloudquotas-v1",
        ))]
        pub mod cloudquotas {
            #[cfg(any(
                feature = "google-api-cloudquotas-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-api-cloudquotas-v1",
                ))]
                include!("bytes_btree_map/google.api.cloudquotas.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-geo-type",
    ))]
    pub mod geo {
        #[cfg(any(
            feature = "google-geo-type",
        ))]
        pub mod r#type {
            #[cfg(any(
                feature = "google-geo-type",
            ))]
            include!("bytes_btree_map/google.geo.r#type.rs");
        }
    }
    #[cfg(any(
        feature = "google-firestore-admin-v1",
        feature = "google-firestore-admin-v1beta1",
        feature = "google-firestore-admin-v1beta2",
        feature = "google-firestore-bundle",
        feature = "google-firestore-v1",
        feature = "google-firestore-v1beta1",
    ))]
    pub mod firestore {
        #[cfg(any(
            feature = "google-firestore-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-firestore-v1",
            ))]
            include!("bytes_btree_map/google.firestore.v1.rs");
        }
        #[cfg(any(
            feature = "google-firestore-bundle",
        ))]
        pub mod bundle {
            #[cfg(any(
                feature = "google-firestore-bundle",
            ))]
            include!("bytes_btree_map/google.firestore.bundle.rs");
        }
        #[cfg(any(
            feature = "google-firestore-admin-v1",
            feature = "google-firestore-admin-v1beta1",
            feature = "google-firestore-admin-v1beta2",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-firestore-admin-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-firestore-admin-v1",
                ))]
                include!("bytes_btree_map/google.firestore.admin.v1.rs");
            }
            #[cfg(any(
                feature = "google-firestore-admin-v1beta2",
            ))]
            pub mod v1beta2 {
                #[cfg(any(
                    feature = "google-firestore-admin-v1beta2",
                ))]
                include!("bytes_btree_map/google.firestore.admin.v1beta2.rs");
            }
            #[cfg(any(
                feature = "google-firestore-admin-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-firestore-admin-v1beta1",
                ))]
                include!("bytes_btree_map/google.firestore.admin.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-firestore-v1beta1",
        ))]
        pub mod v1beta1 {
            #[cfg(any(
                feature = "google-firestore-v1beta1",
            ))]
            include!("bytes_btree_map/google.firestore.v1beta1.rs");
        }
    }
    #[cfg(any(
        feature = "google-iam-admin-v1",
        feature = "google-iam-credentials-v1",
        feature = "google-iam-v1",
        feature = "google-iam-v1-logging",
        feature = "google-iam-v1beta",
        feature = "google-iam-v2",
        feature = "google-iam-v2beta",
    ))]
    pub mod iam {
        #[cfg(any(
            feature = "google-iam-v2",
        ))]
        pub mod v2 {
            #[cfg(any(
                feature = "google-iam-v2",
            ))]
            include!("bytes_btree_map/google.iam.v2.rs");
        }
        #[cfg(any(
            feature = "google-iam-v1",
            feature = "google-iam-v1-logging",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-iam-v1",
            ))]
            include!("bytes_btree_map/google.iam.v1.rs");
            #[cfg(any(
                feature = "google-iam-v1-logging",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-iam-v1-logging",
                ))]
                include!("bytes_btree_map/google.iam.v1.logging.rs");
            }
        }
        #[cfg(any(
            feature = "google-iam-v2beta",
        ))]
        pub mod v2beta {
            #[cfg(any(
                feature = "google-iam-v2beta",
            ))]
            include!("bytes_btree_map/google.iam.v2beta.rs");
        }
        #[cfg(any(
            feature = "google-iam-admin-v1",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-iam-admin-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-iam-admin-v1",
                ))]
                include!("bytes_btree_map/google.iam.admin.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-iam-credentials-v1",
        ))]
        pub mod credentials {
            #[cfg(any(
                feature = "google-iam-credentials-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-iam-credentials-v1",
                ))]
                include!("bytes_btree_map/google.iam.credentials.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-iam-v1beta",
        ))]
        pub mod v1beta {
            #[cfg(any(
                feature = "google-iam-v1beta",
            ))]
            include!("bytes_btree_map/google.iam.v1beta.rs");
        }
    }
    #[cfg(any(
        feature = "google-firebase-fcm-connection-v1alpha1",
    ))]
    pub mod firebase {
        #[cfg(any(
            feature = "google-firebase-fcm-connection-v1alpha1",
        ))]
        pub mod fcm {
            #[cfg(any(
                feature = "google-firebase-fcm-connection-v1alpha1",
            ))]
            pub mod connection {
                #[cfg(any(
                    feature = "google-firebase-fcm-connection-v1alpha1",
                ))]
                pub mod v1alpha1 {
                    #[cfg(any(
                        feature = "google-firebase-fcm-connection-v1alpha1",
                    ))]
                    include!("bytes_btree_map/google.firebase.fcm.connection.v1alpha1.rs");
                }
            }
        }
    }
    #[cfg(any(
        feature = "google-assistant-embedded-v1alpha1",
        feature = "google-assistant-embedded-v1alpha2",
    ))]
    pub mod assistant {
        #[cfg(any(
            feature = "google-assistant-embedded-v1alpha1",
            feature = "google-assistant-embedded-v1alpha2",
        ))]
        pub mod embedded {
            #[cfg(any(
                feature = "google-assistant-embedded-v1alpha2",
            ))]
            pub mod v1alpha2 {
                #[cfg(any(
                    feature = "google-assistant-embedded-v1alpha2",
                ))]
                include!("bytes_btree_map/google.assistant.embedded.v1alpha2.rs");
            }
            #[cfg(any(
                feature = "google-assistant-embedded-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-assistant-embedded-v1alpha1",
                ))]
                include!("bytes_btree_map/google.assistant.embedded.v1alpha1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-chromeos-moblab-v1beta1",
        feature = "google-chromeos-uidetection-v1",
    ))]
    pub mod chromeos {
        #[cfg(any(
            feature = "google-chromeos-moblab-v1beta1",
        ))]
        pub mod moblab {
            #[cfg(any(
                feature = "google-chromeos-moblab-v1beta1",
            ))]
            pub mod v1beta1 {
                #[cfg(any(
                    feature = "google-chromeos-moblab-v1beta1",
                ))]
                include!("bytes_btree_map/google.chromeos.moblab.v1beta1.rs");
            }
        }
        #[cfg(any(
            feature = "google-chromeos-uidetection-v1",
        ))]
        pub mod uidetection {
            #[cfg(any(
                feature = "google-chromeos-uidetection-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-chromeos-uidetection-v1",
                ))]
                include!("bytes_btree_map/google.chromeos.uidetection.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-networking-trafficdirector-type",
    ))]
    pub mod networking {
        #[cfg(any(
            feature = "google-networking-trafficdirector-type",
        ))]
        pub mod trafficdirector {
            #[cfg(any(
                feature = "google-networking-trafficdirector-type",
            ))]
            pub mod r#type {
                #[cfg(any(
                    feature = "google-networking-trafficdirector-type",
                ))]
                include!("bytes_btree_map/google.networking.trafficdirector.r#type.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-container-v1",
        feature = "google-container-v1alpha1",
        feature = "google-container-v1beta1",
    ))]
    pub mod container {
        #[cfg(any(
            feature = "google-container-v1alpha1",
        ))]
        pub mod v1alpha1 {
            #[cfg(any(
                feature = "google-container-v1alpha1",
            ))]
            include!("bytes_btree_map/google.container.v1alpha1.rs");
        }
        #[cfg(any(
            feature = "google-container-v1beta1",
        ))]
        pub mod v1beta1 {
            #[cfg(any(
                feature = "google-container-v1beta1",
            ))]
            include!("bytes_btree_map/google.container.v1beta1.rs");
        }
        #[cfg(any(
            feature = "google-container-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-container-v1",
            ))]
            include!("bytes_btree_map/google.container.v1.rs");
        }
    }
    #[cfg(any(
        feature = "google-identity-accesscontextmanager-type",
        feature = "google-identity-accesscontextmanager-v1",
    ))]
    pub mod identity {
        #[cfg(any(
            feature = "google-identity-accesscontextmanager-type",
            feature = "google-identity-accesscontextmanager-v1",
        ))]
        pub mod accesscontextmanager {
            #[cfg(any(
                feature = "google-identity-accesscontextmanager-type",
            ))]
            pub mod r#type {
                #[cfg(any(
                    feature = "google-identity-accesscontextmanager-type",
                ))]
                include!("bytes_btree_map/google.identity.accesscontextmanager.r#type.rs");
            }
            #[cfg(any(
                feature = "google-identity-accesscontextmanager-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-identity-accesscontextmanager-v1",
                ))]
                include!("bytes_btree_map/google.identity.accesscontextmanager.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-appengine-legacy",
        feature = "google-appengine-logging-v1",
        feature = "google-appengine-v1",
        feature = "google-appengine-v1beta",
    ))]
    pub mod appengine {
        #[cfg(any(
            feature = "google-appengine-logging-v1",
        ))]
        pub mod logging {
            #[cfg(any(
                feature = "google-appengine-logging-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-appengine-logging-v1",
                ))]
                include!("bytes_btree_map/google.appengine.logging.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-appengine-v1beta",
        ))]
        pub mod v1beta {
            #[cfg(any(
                feature = "google-appengine-v1beta",
            ))]
            include!("bytes_btree_map/google.appengine.v1beta.rs");
        }
        #[cfg(any(
            feature = "google-appengine-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-appengine-v1",
            ))]
            include!("bytes_btree_map/google.appengine.v1.rs");
        }
        #[cfg(any(
            feature = "google-appengine-legacy",
        ))]
        pub mod legacy {
            #[cfg(any(
                feature = "google-appengine-legacy",
            ))]
            include!("bytes_btree_map/google.appengine.legacy.rs");
        }
    }
    #[cfg(any(
        feature = "google-chat-logging-v1",
        feature = "google-chat-v1",
    ))]
    pub mod chat {
        #[cfg(any(
            feature = "google-chat-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-chat-v1",
            ))]
            include!("bytes_btree_map/google.chat.v1.rs");
        }
        #[cfg(any(
            feature = "google-chat-logging-v1",
        ))]
        pub mod logging {
            #[cfg(any(
                feature = "google-chat-logging-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-chat-logging-v1",
                ))]
                include!("bytes_btree_map/google.chat.logging.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-bytestream",
    ))]
    pub mod bytestream {
        #[cfg(any(
            feature = "google-bytestream",
        ))]
        include!("bytes_btree_map/google.bytestream.rs");
    }
    #[cfg(any(
        feature = "google-area120-tables-v1alpha1",
    ))]
    pub mod area120 {
        #[cfg(any(
            feature = "google-area120-tables-v1alpha1",
        ))]
        pub mod tables {
            #[cfg(any(
                feature = "google-area120-tables-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-area120-tables-v1alpha1",
                ))]
                include!("bytes_btree_map/google.area120.tables.v1alpha1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-streetview-publish-v1",
    ))]
    pub mod streetview {
        #[cfg(any(
            feature = "google-streetview-publish-v1",
        ))]
        pub mod publish {
            #[cfg(any(
                feature = "google-streetview-publish-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-streetview-publish-v1",
                ))]
                include!("bytes_btree_map/google.streetview.publish.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-pubsub-v1",
        feature = "google-pubsub-v1beta2",
    ))]
    pub mod pubsub {
        #[cfg(any(
            feature = "google-pubsub-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-pubsub-v1",
            ))]
            include!("bytes_btree_map/google.pubsub.v1.rs");
        }
        #[cfg(any(
            feature = "google-pubsub-v1beta2",
        ))]
        pub mod v1beta2 {
            #[cfg(any(
                feature = "google-pubsub-v1beta2",
            ))]
            include!("bytes_btree_map/google.pubsub.v1beta2.rs");
        }
    }
    #[cfg(any(
        feature = "google-monitoring-dashboard-v1",
        feature = "google-monitoring-metricsscope-v1",
        feature = "google-monitoring-v3",
    ))]
    pub mod monitoring {
        #[cfg(any(
            feature = "google-monitoring-metricsscope-v1",
        ))]
        pub mod metricsscope {
            #[cfg(any(
                feature = "google-monitoring-metricsscope-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-monitoring-metricsscope-v1",
                ))]
                include!("bytes_btree_map/google.monitoring.metricsscope.v1.rs");
            }
        }
        #[cfg(any(
            feature = "google-monitoring-v3",
        ))]
        pub mod v3 {
            #[cfg(any(
                feature = "google-monitoring-v3",
            ))]
            include!("bytes_btree_map/google.monitoring.v3.rs");
        }
        #[cfg(any(
            feature = "google-monitoring-dashboard-v1",
        ))]
        pub mod dashboard {
            #[cfg(any(
                feature = "google-monitoring-dashboard-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-monitoring-dashboard-v1",
                ))]
                include!("bytes_btree_map/google.monitoring.dashboard.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-storage-control-v2",
        feature = "google-storage-v1",
        feature = "google-storage-v2",
    ))]
    pub mod storage {
        #[cfg(any(
            feature = "google-storage-control-v2",
        ))]
        pub mod control {
            #[cfg(any(
                feature = "google-storage-control-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-storage-control-v2",
                ))]
                include!("bytes_btree_map/google.storage.control.v2.rs");
            }
        }
        #[cfg(any(
            feature = "google-storage-v2",
        ))]
        pub mod v2 {
            #[cfg(any(
                feature = "google-storage-v2",
            ))]
            include!("bytes_btree_map/google.storage.v2.rs");
        }
        #[cfg(any(
            feature = "google-storage-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-storage-v1",
            ))]
            include!("bytes_btree_map/google.storage.v1.rs");
        }
    }
    #[cfg(any(
        feature = "google-search-partnerdataingestion-logging-v1",
    ))]
    pub mod search {
        #[cfg(any(
            feature = "google-search-partnerdataingestion-logging-v1",
        ))]
        pub mod partnerdataingestion {
            #[cfg(any(
                feature = "google-search-partnerdataingestion-logging-v1",
            ))]
            pub mod logging {
                #[cfg(any(
                    feature = "google-search-partnerdataingestion-logging-v1",
                ))]
                pub mod v1 {
                    #[cfg(any(
                        feature = "google-search-partnerdataingestion-logging-v1",
                    ))]
                    include!("bytes_btree_map/google.search.partnerdataingestion.logging.v1.rs");
                }
            }
        }
    }
    #[cfg(any(
        feature = "google-example-library-v1",
    ))]
    pub mod example {
        #[cfg(any(
            feature = "google-example-library-v1",
        ))]
        pub mod library {
            #[cfg(any(
                feature = "google-example-library-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "google-example-library-v1",
                ))]
                include!("bytes_btree_map/google.example.library.v1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-dataflow-v1beta3",
    ))]
    pub mod dataflow {
        #[cfg(any(
            feature = "google-dataflow-v1beta3",
        ))]
        pub mod v1beta3 {
            #[cfg(any(
                feature = "google-dataflow-v1beta3",
            ))]
            include!("bytes_btree_map/google.dataflow.v1beta3.rs");
        }
    }
    #[cfg(any(
        feature = "google-storagetransfer-logging",
        feature = "google-storagetransfer-v1",
    ))]
    pub mod storagetransfer {
        #[cfg(any(
            feature = "google-storagetransfer-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-storagetransfer-v1",
            ))]
            include!("bytes_btree_map/google.storagetransfer.v1.rs");
        }
        #[cfg(any(
            feature = "google-storagetransfer-logging",
        ))]
        pub mod logging {
            #[cfg(any(
                feature = "google-storagetransfer-logging",
            ))]
            include!("bytes_btree_map/google.storagetransfer.logging.rs");
        }
    }
    #[cfg(any(
        feature = "google-bigtable-admin-v2",
        feature = "google-bigtable-v2",
    ))]
    pub mod bigtable {
        #[cfg(any(
            feature = "google-bigtable-v2",
        ))]
        pub mod v2 {
            #[cfg(any(
                feature = "google-bigtable-v2",
            ))]
            include!("bytes_btree_map/google.bigtable.v2.rs");
        }
        #[cfg(any(
            feature = "google-bigtable-admin-v2",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-bigtable-admin-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-bigtable-admin-v2",
                ))]
                include!("bytes_btree_map/google.bigtable.admin.v2.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-marketingplatform-admin-v1alpha",
    ))]
    pub mod marketingplatform {
        #[cfg(any(
            feature = "google-marketingplatform-admin-v1alpha",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-marketingplatform-admin-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-marketingplatform-admin-v1alpha",
                ))]
                include!("bytes_btree_map/google.marketingplatform.admin.v1alpha.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-partner-aistreams-v1alpha1",
    ))]
    pub mod partner {
        #[cfg(any(
            feature = "google-partner-aistreams-v1alpha1",
        ))]
        pub mod aistreams {
            #[cfg(any(
                feature = "google-partner-aistreams-v1alpha1",
            ))]
            pub mod v1alpha1 {
                #[cfg(any(
                    feature = "google-partner-aistreams-v1alpha1",
                ))]
                include!("bytes_btree_map/google.partner.aistreams.v1alpha1.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-rpc",
        feature = "google-rpc-context",
    ))]
    pub mod rpc {
        #[cfg(any(
            feature = "google-rpc",
        ))]
        include!("bytes_btree_map/google.rpc.rs");
        #[cfg(any(
            feature = "google-rpc-context",
        ))]
        pub mod context {
            #[cfg(any(
                feature = "google-rpc-context",
            ))]
            include!("bytes_btree_map/google.rpc.context.rs");
        }
    }
    #[cfg(any(
        feature = "google-analytics-admin-v1alpha",
        feature = "google-analytics-admin-v1beta",
        feature = "google-analytics-data-v1alpha",
        feature = "google-analytics-data-v1beta",
    ))]
    pub mod analytics {
        #[cfg(any(
            feature = "google-analytics-admin-v1alpha",
            feature = "google-analytics-admin-v1beta",
        ))]
        pub mod admin {
            #[cfg(any(
                feature = "google-analytics-admin-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-analytics-admin-v1beta",
                ))]
                include!("bytes_btree_map/google.analytics.admin.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-analytics-admin-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-analytics-admin-v1alpha",
                ))]
                include!("bytes_btree_map/google.analytics.admin.v1alpha.rs");
            }
        }
        #[cfg(any(
            feature = "google-analytics-data-v1alpha",
            feature = "google-analytics-data-v1beta",
        ))]
        pub mod data {
            #[cfg(any(
                feature = "google-analytics-data-v1beta",
            ))]
            pub mod v1beta {
                #[cfg(any(
                    feature = "google-analytics-data-v1beta",
                ))]
                include!("bytes_btree_map/google.analytics.data.v1beta.rs");
            }
            #[cfg(any(
                feature = "google-analytics-data-v1alpha",
            ))]
            pub mod v1alpha {
                #[cfg(any(
                    feature = "google-analytics-data-v1alpha",
                ))]
                include!("bytes_btree_map/google.analytics.data.v1alpha.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-privacy-dlp-v2",
    ))]
    pub mod privacy {
        #[cfg(any(
            feature = "google-privacy-dlp-v2",
        ))]
        pub mod dlp {
            #[cfg(any(
                feature = "google-privacy-dlp-v2",
            ))]
            pub mod v2 {
                #[cfg(any(
                    feature = "google-privacy-dlp-v2",
                ))]
                include!("bytes_btree_map/google.privacy.dlp.v2.rs");
            }
        }
    }
    #[cfg(any(
        feature = "google-watcher-v1",
    ))]
    pub mod watcher {
        #[cfg(any(
            feature = "google-watcher-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "google-watcher-v1",
            ))]
            include!("bytes_btree_map/google.watcher.v1.rs");
        }
    }
    #[cfg(any(
        feature = "google-gapic-metadata",
    ))]
    pub mod gapic {
        #[cfg(any(
            feature = "google-gapic-metadata",
        ))]
        pub mod metadata {
            #[cfg(any(
                feature = "google-gapic-metadata",
            ))]
            include!("bytes_btree_map/google.gapic.metadata.rs");
        }
    }
}
#[cfg(any(
    feature = "maps-fleetengine-delivery-v1",
    feature = "maps-fleetengine-v1",
))]
pub mod maps {
    #[cfg(any(
        feature = "maps-fleetengine-delivery-v1",
        feature = "maps-fleetengine-v1",
    ))]
    pub mod fleetengine {
        #[cfg(any(
            feature = "maps-fleetengine-v1",
        ))]
        pub mod v1 {
            #[cfg(any(
                feature = "maps-fleetengine-v1",
            ))]
            include!("bytes_btree_map/maps.fleetengine.v1.rs");
        }
        #[cfg(any(
            feature = "maps-fleetengine-delivery-v1",
        ))]
        pub mod delivery {
            #[cfg(any(
                feature = "maps-fleetengine-delivery-v1",
            ))]
            pub mod v1 {
                #[cfg(any(
                    feature = "maps-fleetengine-delivery-v1",
                ))]
                include!("bytes_btree_map/maps.fleetengine.delivery.v1.rs");
            }
        }
    }
}
#[cfg(any(
    feature = "grafeas-v1",
    feature = "grafeas-v1beta1",
    feature = "grafeas-v1beta1-attestation",
    feature = "grafeas-v1beta1-build",
    feature = "grafeas-v1beta1-deployment",
    feature = "grafeas-v1beta1-discovery",
    feature = "grafeas-v1beta1-image",
    feature = "grafeas-v1beta1-package",
    feature = "grafeas-v1beta1-provenance",
    feature = "grafeas-v1beta1-source",
    feature = "grafeas-v1beta1-vulnerability",
))]
pub mod grafeas {
    #[cfg(any(
        feature = "grafeas-v1beta1",
        feature = "grafeas-v1beta1-attestation",
        feature = "grafeas-v1beta1-build",
        feature = "grafeas-v1beta1-deployment",
        feature = "grafeas-v1beta1-discovery",
        feature = "grafeas-v1beta1-image",
        feature = "grafeas-v1beta1-package",
        feature = "grafeas-v1beta1-provenance",
        feature = "grafeas-v1beta1-source",
        feature = "grafeas-v1beta1-vulnerability",
    ))]
    pub mod v1beta1 {
        #[cfg(any(
            feature = "grafeas-v1beta1",
        ))]
        include!("bytes_btree_map/grafeas.v1beta1.rs");
        #[cfg(any(
            feature = "grafeas-v1beta1-discovery",
        ))]
        pub mod discovery {
            #[cfg(any(
                feature = "grafeas-v1beta1-discovery",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.discovery.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-provenance",
        ))]
        pub mod provenance {
            #[cfg(any(
                feature = "grafeas-v1beta1-provenance",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.provenance.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-source",
        ))]
        pub mod source {
            #[cfg(any(
                feature = "grafeas-v1beta1-source",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.source.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-vulnerability",
        ))]
        pub mod vulnerability {
            #[cfg(any(
                feature = "grafeas-v1beta1-vulnerability",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.vulnerability.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-deployment",
        ))]
        pub mod deployment {
            #[cfg(any(
                feature = "grafeas-v1beta1-deployment",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.deployment.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-package",
        ))]
        pub mod package {
            #[cfg(any(
                feature = "grafeas-v1beta1-package",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.package.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-attestation",
        ))]
        pub mod attestation {
            #[cfg(any(
                feature = "grafeas-v1beta1-attestation",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.attestation.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-image",
        ))]
        pub mod image {
            #[cfg(any(
                feature = "grafeas-v1beta1-image",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.image.rs");
        }
        #[cfg(any(
            feature = "grafeas-v1beta1-build",
        ))]
        pub mod build {
            #[cfg(any(
                feature = "grafeas-v1beta1-build",
            ))]
            include!("bytes_btree_map/grafeas.v1beta1.build.rs");
        }
    }
    #[cfg(any(
        feature = "grafeas-v1",
    ))]
    pub mod v1 {
        #[cfg(any(
            feature = "grafeas-v1",
        ))]
        include!("bytes_btree_map/grafeas.v1.rs");
    }
}
#[cfg(any(
    feature = "cloud-kubernetes-security-containersecurity_logging",
))]
pub mod cloud {
    #[cfg(any(
        feature = "cloud-kubernetes-security-containersecurity_logging",
    ))]
    pub mod kubernetes {
        #[cfg(any(
            feature = "cloud-kubernetes-security-containersecurity_logging",
        ))]
        pub mod security {
            #[cfg(any(
                feature = "cloud-kubernetes-security-containersecurity_logging",
            ))]
            pub mod containersecurity_logging {
                #[cfg(any(
                    feature = "cloud-kubernetes-security-containersecurity_logging",
                ))]
                include!("bytes_btree_map/cloud.kubernetes.security.containersecurity_logging.rs");
            }
        }
    }
}
