<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Ins</name>
   <tag></tag>
   <elementGuidId>f9eb7938-6b1b-40a1-983f-ef97b6becfd9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Request\&quot; : {\n               \&quot;App_Test2___Sys_Dic\&quot; : [\n                                 {\n                                 \&quot;OwnerUser\&quot; : 1,\n                                 \&quot;InsUser\&quot; : 1,\n                                 \&quot;ModUser\&quot; : 1,\n                                 \&quot;InsDate\&quot; : \&quot;2019-06-20\&quot;,\n                                 \&quot;ModDate\&quot; : \&quot;2019-06-20\&quot;,\n                                 \&quot;Name\&quot; : \&quot;Name\&quot;,\n                                 \&quot;Value_Boolean\&quot; : 1,\n                                 \&quot;Name1\&quot; : \&quot;Name1\&quot;,\n                                 \&quot;App_Test2___Sys_Table\&quot; : null,\n                                 \&quot;App_Test2___Sys_Doc\&quot; : null\n                                 \t\n                                 }\n                               ]\n\n              }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookie}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/wapi/App_Test2___Sys_Dic/Ins</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>e1d9ef16-85c6-4bc9-92e0-ffdb2f924f91</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookie</defaultValue>
      <description></description>
      <id>84f2afdd-0719-491f-a172-51870e3311b4</id>
      <masked>false</masked>
      <name>cookie</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

print(GlobalVariable.cookie)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
