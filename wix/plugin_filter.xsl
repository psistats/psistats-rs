<xsl:stylesheet 
    version="1.0"
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:wix="http://schemas.microsoft.com/wix/2006/wi" 
    xmlns="http://schemas.microsoft.com/wix/2006/wi">

    <!--
    <Fragment>
        <ComponentGroup Id="PsistatsPlugins">
    -->
    <xsl:template match="wix:Fragment/wix:ComponentGroup[@Id='PsistatsPlugins']">
    <Fragment>
        <ComponentGroup Id="PsistatsPlugins">
            <!--<xsl:for-each select="//wix:Component[starts-with(wix:File/@Source, 'SourceDir\plugin_')]">-->
            <!--<xsl:for-each select="//wix:Component[starts-with(wix:File/@Id, 'plugin')]">-->
            <xsl:for-each select="//wix:Component[
                starts-with(wix:File/@Source, 'SourceDir\plugin_') 
                and 
                '.dll' = substring(wix:File/@Source, string-length(wix:File/@Source) - string-length('.dll') +1)]
            ">
                <xsl:variable name="plugin_name" select="substring-before(substring-after(wix:File/@Source, 'SourceDir\plugin_'), '.dll')"/>
                <ComponentRef Id="plugin_{$plugin_name}_cmp"/>


                
            </xsl:for-each>
        </ComponentGroup>
    </Fragment>
    </xsl:template>


    <!--
                <Component Id="cmp537082604604EEFD72DFC89BC0642564" Guid="{F266C206-DDD4-494D-84C7-71438153952F}">
                    <File Id="fil13B611F46534628E2180255D986E4EEE" KeyPath="yes" Source="SourceDir\plugin_cpu.dll" />
                </Component>    
    -->
    <xsl:template match="wix:Component[
                starts-with(wix:File/@Source, 'SourceDir\plugin_') 
                and 
                '.dll' = substring(wix:File/@Source, string-length(wix:File/@Source) - string-length('.dll') +1)]">
        <xsl:variable name="plugin_name" select="substring-before(substring-after(wix:File/@Source, 'SourceDir\plugin_'), '.dll')"/>
        <Component Id="plugin_{$plugin_name}_cmp" Guid="*" Win64="yes">
            <File 
                Id="plugin_{$plugin_name}_dll" 
                KeyPath="yes" 
                Source="target\release\unzipped\plugins\plugin_{$plugin_name}.dll"
                Name="plugin_{$plugin_name}.dll"
                />
        </Component>
    </xsl:template>
    
    <xsl:template match="wix:DirectoryRef[@Id='plugins']">
    <Fragment>
        <DirectoryRef Id="PLUGINSFOLDER">
            <xsl:apply-templates select="//wix:Component[
            starts-with(wix:File/@Source, 'SourceDir\plugin_') 
            and 
            '.dll' = substring(wix:File/@Source, string-length(wix:File/@Source) - string-length('.dll') +1)]"/>  
        <!--
            <xsl:for-each select="wix:Directory/wix:Component[contains(concat(wix:File/@Source,'|'), '.dll|')]">
                <xsl:param name="plugin_name" select="wix:File/@Source"/>
                <xsl:value-of select="$plugin_name"/>
            </xsl:for-each>
        -->
        </DirectoryRef>
    </Fragment>

    </xsl:template>


    

    <xsl:template match="wix:Directory/wix:Component[not(
        contains(concat(wix:File/@Source,'|'), '.dll|'))]">
    </xsl:template>
    <!-- <xsl:template match="wix:Directory"></xsl:template> -->

<xsl:template match="/">

<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
    <xsl:apply-templates />
</Wix>

</xsl:template>
<!--
            <xsl:copy>
              <xsl:apply-templates select="@*|node()"/>
            </xsl:copy>
          </xsl:template>
          -->
 </xsl:stylesheet>
