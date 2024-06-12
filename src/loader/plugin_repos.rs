pub struct Plugin {
    pub name: &'static str,
    pub url: &'static str,
    pub file_name: &'static str,
}

const ARC_DPS_PLUGIN: Plugin = Plugin{
    name: "arc_dps",
    url: "https://www.deltaconnected.com/arcdps/x64/d3d11.dll",
    file_name: "d3d11.dll",
};

pub const ALL_PLUGINS: [Plugin; 1] = [ARC_DPS_PLUGIN];
