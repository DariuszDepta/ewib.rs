pub const VALIDATION_1_DMN_TEMPLATE: &str = r###"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<definitions xmlns="https://www.omg.org/spec/DMN/20191111/MODEL/"
    namespace="https://ewib/validation-1"
    name="ewib-validation-1"
    id="_f39bb092-87e0-4c07-94fb-378ab311ce67">
    <description>Walidacje oparte na EWIB</description>
    <decision name="Nazwa Instytucji" id="_5e1ef897-b64b-4d2e-8f49-3b35a783347b">
        <variable typeRef="string" name="Nazwa Instytucji"/>
        <informationRequirement>
            <requiredInput href="#_c495e611-1de6-40aa-8729-6b992afd19c5"/>
        </informationRequirement>
        <decisionTable outputLabel="Nazwa Instytucji">
            <input label="NrRozliczeniowy">
                <inputExpression typeRef="string">
                    <text>NrRozliczeniowy</text>
                </inputExpression>
            </input>
            <output label="Nazwa Instytucji"/>
#RULES#        </decisionTable>
    </decision>
    <inputData name="NrRozliczeniowy" id="_c495e611-1de6-40aa-8729-6b992afd19c5">
        <variable typeRef="string" name="NrRozliczeniowy"/>
    </inputData>
</definitions>
"###;

pub const RULE_TEMPLATE: &str = r###"            <rule>
                <inputEntry>
                    <text>#NR#</text>
                </inputEntry>
                <outputEntry>
                    <text>#NI#</text>
                </outputEntry>
            </rule>"###;
