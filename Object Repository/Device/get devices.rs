<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get devices</name>
   <tag></tag>
   <elementGuidId>dc2f6301-b4c5-49d1-8b5a-cb6901f972c8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${authorization}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.kobiton.com/v1/devices</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>17b6e0d5-6533-4b52-8432-c4f9421e66ef</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import groovy.json.internal.LazyMap

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

WS.verifyResponseStatusCode(response, 200)

assertThat(jsonResponse.privateDevices).isNotNull()
assertThat(jsonResponse.favoriteDevices).isNotNull()
assertThat(jsonResponse.cloudDevices).isNotNull()

assertThat(jsonResponse.privateDevices[0].id).isInstanceOf(Integer)
assertThat(jsonResponse.privateDevices[0].udid).isNotEmpty()
assertThat(jsonResponse.privateDevices[0].isBooked).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].isHidden).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].isOnline).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].modelName).isInstanceOf(String)
assertThat(jsonResponse.privateDevices[0].deviceName).isInstanceOf(String)

assertThat(jsonResponse.privateDevices[0].resolution).isInstanceOf(LazyMap)
assertThat(jsonResponse.privateDevices[0].resolution.width).isInstanceOf(Integer)
assertThat(jsonResponse.privateDevices[0].resolution.height).isInstanceOf(Integer)

assertThat(jsonResponse.privateDevices[0].platformName).isInstanceOf(String)
assertThat(jsonResponse.privateDevices[0].platformVersion).isInstanceOf(String)

assertThat(jsonResponse.privateDevices[0].installedBrowsers).isInstanceOf(ArrayList)
assertThat(jsonResponse.privateDevices[0].installedBrowsers[0].name).isInstanceOf(String)
assertThat(jsonResponse.privateDevices[0].installedBrowsers[0].version).isInstanceOf(String)

assertThat(jsonResponse.privateDevices[0].support).isInstanceOf(LazyMap)
assertThat(jsonResponse.privateDevices[0].support.appiumDisabled).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].support.setTimeZoneDisabled).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].support.longPressHomeButtonDisabled).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].support.doublePressHomeButtonDisabled).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].support.networkTrafficCapturingDisabled).isInstanceOf(Boolean)

assertThat(jsonResponse.privateDevices[0].deviceImageUrl).isInstanceOf(String)
assertThat(jsonResponse.privateDevices[0].isFavorite).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].isCloud).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].isMyOrg).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].isMyOwn).isInstanceOf(Boolean)
assertThat(jsonResponse.privateDevices[0].hostedBy).isInstanceOf(String)








</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
