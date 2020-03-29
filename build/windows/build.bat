SET PATH_SELF=%~dp0
SET PROJECT_VERSION="0.2.0"

if "%APPVEYOR_BUILD_NUMBER%"=="" ( 
    echo "Not an appveyor build"
) ELSE ( 
    set PROJECT_VERSION=%PROJECT_VERISON%.%APPVEYOR_BUILD_NUMBER%
)

pushd .
cd %PATH_SELF:~0,-1%\..\..
set PROJECT_PATH=%CD%
popd

echo Project path: %PROJECT_PATH%

cd %PROJECT_PATH%
cargo build --release
mkdir %PROJECT_PATH%\target\release\artifact
mkdir %PROJECT_PATH%\target\release\artifact\plugins
copy %PROJECT_PATH%\target\release\psistats.exe %PROJECT_PATH%\target\release\artifact\psistats.exe
copy %PROJECT_PATH%\target\release\plugin_*.dll %PROJECT_PATH%\target\release\artifact\plugins
7z a %PROJECT_PATH%\target\release\psistats-%PROJECT_VERSION%.zip %PROJECT_PATH%\target\release\artifact\*

heat dir target\release\artifact\plugins -cg PsistatsPlugins -gg -out target\wix\plugins.wxs -t wix\plugin_filter.xsl -dr plugins
cargo wix --name psistats -i %PROJECT_VERSION% --include %PROJECT_PATH%\target\wix\plugins.wxs --nocapture
