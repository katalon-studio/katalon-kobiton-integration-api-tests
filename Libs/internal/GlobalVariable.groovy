package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object authorization
     
    /**
     * <p></p>
     */
    public static Object token
     

    static {
        def allVariables = [:]        
        allVariables.put('default', ['authorization' : 'Basic a2F0YWxvbi1pbnRlZ3JhdGlvbjpkODYxMTk0Ni04MjM5LTRhZTMtYWY0Yi1jNzllOGVmMmU3N2Q=', 'token' : 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MTU4MzIsImlhdCI6MTU0MDM4NTM0NywiZXhwIjoxNTQyOTc3MzQ3fQ.y2xZzaK2zJKBLbBTG6ai1NvgjYcucW3_BvrrS-cHre8'])
        
        String profileName = RunConfiguration.getExecutionProfile()
        
        def selectedVariables = allVariables[profileName]
        authorization = selectedVariables['authorization']
        token = selectedVariables['token']
        
    }
}
