<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get app</name>
   <tag></tag>
   <elementGuidId>a741f5fc-762d-4a5b-b087-e648bf768d5d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${authorization}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.kobiton.com/v1/apps</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>e0827c5b-070a-4510-ba66-091d8a090754</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import groovy.json.internal.LazyMap

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

assertThat(jsonResponse.apps[0].id).isInstanceOf(Integer).isNotNull()
assertThat(jsonResponse.apps[0].createdAt).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].name).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].privateAccess).isInstanceOf(Boolean).isNotNull()
assertThat(jsonResponse.apps[0].os).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].createdBy).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].state).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].bypas)
assertThat(jsonResponse.apps[0].organizationId)

assertThat(jsonResponse.apps[0].versions).isInstanceOf(ArrayList).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].id).isInstanceOf(Integer).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].createdAt).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].name).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].version).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].createdBy).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].state).isInstanceOf(String).isNotNull()

assertThat(jsonResponse.apps[0].versions[0].nativeProperties).isInstanceOf(LazyMap).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].nativeProperties.DTXcode).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].nativeProperties.DTSDKName).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].nativeProperties.DTCompiler).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].nativeProperties.DTSDKBuild).isInstanceOf(String).isNotNull()
assertThat(jsonResponse.apps[0].versions[0].nativeProperties.UIAppFonts).isInstanceOf(ArrayList).isNotNull()



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
