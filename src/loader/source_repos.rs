pub struct Source {
    pub name: &'static str,
    pub url: &'static str,
}

const SNOW_CROWS: Source = Source {
    name: "Snow Crows",
    url: "https://snowcrows.com/builds/raids",
};

pub const ALL_SOURCES: [Source; 1] = [SNOW_CROWS];
