<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search API Copy</name>
   <tag></tag>
   <elementGuidId>d4214c24-654c-4a9e-b044-9064074865d0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;pagination\&quot;:{\n        \&quot;page\&quot;:0,\n        \&quot;size\&quot;:1,\n        \&quot;sorts\&quot;:\&quot;order,desc\&quot;\n    },\n    \&quot;conditions\&quot;:[\n        {\n            \&quot;key\&quot;:\&quot;Project.id\&quot;,\n            \&quot;operator\&quot;:\&quot;\u003d\&quot;,\n            \&quot;value\&quot;:\&quot;83798\&quot;\n        },\n        {\n            \&quot;key\&quot;:\&quot;Project.id\&quot;,\n            \&quot;operator\&quot;:\&quot;\u003d\&quot;,\n            \&quot;value\&quot;:\&quot;83798\&quot;\n        }\n    ],\n    \&quot;type\&quot;:\&quot;ExternalConnection\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dHVhbi5uZ3V5ZW5Aa2F0YWxvbi5jb206c2FkZmRzZmRzZmZmcXJmd2U=</value>
      <webElementGuid>44e5927f-e6c8-4d54-a3b0-6d387cf52028</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://testops.staging.katalon.com/api/v1/search</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
