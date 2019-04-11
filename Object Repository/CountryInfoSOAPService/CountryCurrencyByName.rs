<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Generates country's currency Name</description>
   <name>CountryCurrencyByName</name>
   <tag></tag>
   <elementGuidId>6b44456b-1b82-4b6f-ab7b-06c4a36d0fc9</elementGuidId>
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
      <value>${currAuth}</value>
   </httpHeaderProperties>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;CountryCurrency xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
            &lt;sCountryISOCode>${CountryISO}&lt;/sCountryISOCode>
        &lt;/CountryCurrency>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CurrencyName</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.CountryISO</defaultValue>
      <description></description>
      <id>dffa6945-eb3d-4c14-8c23-92f90e3b8210</id>
      <masked>false</masked>
      <name>CountryISO</name>
   </variables>
   <variables>
      <defaultValue>'Basic Og=='</defaultValue>
      <description></description>
      <id>d844792d-5199-47ff-9c5e-795de541ffef</id>
      <masked>false</masked>
      <name>currAuth</name>
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


WS.verifyElementText(response, 'CountryCurrencyResponse.CountryCurrencyResult.sName', 'Dollars')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
