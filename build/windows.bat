SET PATH_SELF=%~dp0

pushd .
cd %PATH_SELF:~0,-1%\..
set PROJECT_PATH=%CD%
popd

echo Project path: %PROJECT_PATH%

cd %PROJECT_PATH%
cargo build --release
heat dir target\release -cg PsistatsPlugins -gg -out target\wix\plugins.wxs -t wix\plugin_filter.xsl -dr plugins
cargo wix %PROJECT_PATH%\psistats\Cargo.toml --include %PROJECT_PATH%\target\wix\plugins.wxs --nocapture
