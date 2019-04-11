import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

println('Country Code :' + countryCode)

println('Country Capital :' + countryCapital)

println('Country Currency :' + countryCurrency)


cptlResponse = WS.sendRequest(findTestObject('SOAP API Country Info/CapitalCity' , ['CountryCode' : countryCode]))

WS.verifyElementText(cptlResponse, 'CapitalCityResponse.CapitalCityResult', countryCapital)


currResponse = WS.sendRequest(findTestObject('SOAP API Country Info/CountryCurrency', ['CountryCode' : countryCode]))

WS.verifyElementText(currResponse, 'CountryCurrencyResponse.CountryCurrencyResult.sName', countryCurrency)

