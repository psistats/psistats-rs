SET PATH_SELF=%~dp0
SET PROJECT_VERSION=0.3.0-beta
SET BUILD_VERSION=%PROJECT_VERSION%

if "%JENKINS_BUILD_NUMBER%"=="" ( 
    echo "Not an appveyor build"
) ELSE ( 
    set BUILD_VERSION=%BUILD_VERSION%.%JENKINS_BUILD_NUMBER%
)

pushd .
cd %PATH_SELF:~0,-1%\..\..
set PROJECT_PATH=%CD%
popd

echo Project path: %PROJECT_PATH%

cd "%PROJECT_PATH%"
cargo clean
cargo install cargo-wix cargo-config
cargo build --release
mkdir "%PROJECT_PATH%\target\release\unzipped"
mkdir "%PROJECT_PATH%\target\release\unzipped\plugins"
mkdir "%PROJECT_PATH%\target\artifacts"
copy "%PROJECT_PATH%\target\release\psistats.exe" "%PROJECT_PATH%\target\release\unzipped\psistats.exe"
copy "%PROJECT_PATH%\target\release\plugin_*.dll" "%PROJECT_PATH%\target\release\unzipped\plugins"
copy "%PROJECT_PATH%\LICENSE" "%PROJECT_PATH%\target\release\unzipped"
7z a "%PROJECT_PATH%\target\release\artifacts\psistats-%BUILD_VERSION%.zip" "%PROJECT_PATH%\target\release\unzipped"\*

heat dir target\release\unzipped\plugins -cg PsistatsPlugins -gg -out target\wix\plugins.wxs -t "%PROJECT_PATH%\wix\plugin_filter.xsl" -dr plugins
cargo wix --name=psistats --install-version=%PROJECT_VERSION% --include="%PROJECT_PATH%\target\wix\plugins.wxs" --nocapture --output "%PROJECT_PATH%\target\release\artifacts\psistats-%BUILD_VERSION%.msi"
