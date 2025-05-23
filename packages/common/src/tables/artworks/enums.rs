use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]

pub enum Rarity {
    #[default]
    #[translate(ko = "Unique", en = "Unique")]
    Unique = 1,
    #[translate(ko = "Limited Edition", en = "Limited Edition")]
    LimitedEdition = 2,
    #[translate(ko = "Open Edition", en = "Open Edition")]
    OpenEdition = 3,
    #[translate(ko = "Edition", en = "Edition")]
    UnknownEdition = 4,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]

pub enum Status {
    #[default]
    #[translate(ko = "Active", en = "Active")]
    Active = 1,
    #[translate(ko = "Hide", en = "Hide")]
    Hide = 2,
    #[translate(ko = "Pending", en = "Pending")]
    Pending = 3,
    #[translate(ko = "Sold", en = "Sold")]
    Sold = 4,
    #[translate(ko = "Withdrawn", en = "Withdrawn")]
    Withdrawn = 5,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Theme {
    #[default]
    #[translate(ko = "Abstract", en = "Abstract")]
    Abstract = 1,
    #[translate(ko = "Animals", en = "Animals")]
    Animals = 2,
    #[translate(ko = "Fantasy", en = "Fantasy")]
    Fantasy = 3,
    #[translate(ko = "Nature", en = "Nature")]
    Nature = 4,
    #[translate(ko = "Portraits", en = "Portraits")]
    Portraits = 5,
    #[translate(ko = "Sci-Fi", en = "Sci-Fi")]
    SciFi = 6,
    #[translate(ko = "Urban", en = "Urban")]
    Urban = 7,
    #[translate(ko = "Pop", en = "Pop")]
    Pop = 8,
    #[translate(ko = "Culture", en = "Culture")]
    Culture = 9,
    #[translate(ko = "Spirituality", en = "Spirituality")]
    Spirituality = 10,
    #[translate(ko = "Mythology", en = "Mythology")]
    Mythology = 11,
    #[translate(ko = "Technology", en = "Technology")]
    Technology = 12,
    #[translate(ko = "Emotions", en = "Emotions")]
    Emotions = 13,
    #[translate(ko = "Dreams", en = "Dreams")]
    Dreams = 14,
    #[translate(ko = "Adventure", en = "Adventure")]
    Adventure = 15,
    #[translate(ko = "Gaming", en = "Gaming")]
    Gaming = 16,
    #[translate(ko = "Music", en = "Music")]
    Music = 17,
    #[translate(ko = "Sports", en = "Sports")]
    Sports = 18,
    #[translate(ko = "Figurative", en = "Figurative")]
    Figurative = 19,
    #[translate(ko = "Landscape", en = "Landscape")]
    Landscape = 20,
    #[translate(ko = "Heritage", en = "Heritage")]
    Heritage = 21,
    #[translate(ko = "Environment", en = "Environment")]
    Environment = 22,
    #[translate(ko = "Society", en = "Society")]
    Society = 23,
    #[translate(ko = "Commentary", en = "Commentary")]
    Commentary = 24,
    #[translate(ko = "Political", en = "Political")]
    Political = 25,
    #[translate(ko = "Religion", en = "Religion")]
    Religion = 26,
    #[translate(ko = "Innovation", en = "Innovation")]
    Innovation = 27,
    #[translate(ko = "Gender Identity", en = "Gender Identity")]
    GenderIdentity = 28,
    #[translate(ko = "Race", en = "Race")]
    Race = 29,
    #[translate(ko = "Human", en = "Human")]
    Human = 30,
    #[translate(ko = "Architecture", en = "Architecture")]
    Architecture = 31,
    #[translate(ko = "History", en = "History")]
    History = 32,
    #[translate(ko = "Cyberpunk", en = "Cyberpunk")]
    Cyberpunk = 33,
    #[translate(ko = "Steampunk", en = "Steampunk")]
    Steampunk = 34,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]

pub enum Medium {
    #[default]
    #[translate(ko = "Digita Art", en = "Digital Art")]
    DigitalArt = 1,
    #[translate(ko = "Painting", en = "Painting")]
    Painting = 2,
    #[translate(ko = "Drawing", en = "Drawing")]
    Drawing = 3,
    #[translate(ko = "Photography", en = "Photography")]
    Photography = 4,
    #[translate(ko = "Sculpture", en = "Sculpture")]
    Sculpture = 5,
    #[translate(ko = "Print", en = "Print")]
    Print = 6,
    #[translate(ko = "Design", en = "Design")]
    Design = 7,
    #[translate(ko = "Reproduction", en = "Reproduction")]
    Reproduction = 8,
    #[translate(ko = "Performance", en = "Performance")]
    Performance = 9,
    #[translate(ko = "Jewelry", en = "Jewelry")]
    Jewelry = 10,
    #[translate(ko = "Work on Paper", en = "Work on Paper")]
    WorkOnPaper = 11,
    #[translate(ko = "Video/Film", en = "Video/Film")]
    VideoFilm = 12,
    #[translate(ko = "Mixed Media", en = "Mixed Media")]
    MixedMedia = 13,
    #[translate(ko = "Textile", en = "Textile")]
    Textile = 14,
    #[translate(ko = "Installation", en = "Installation")]
    Installation = 15,
    #[translate(ko = "Other", en = "Other")]
    Other = 16,
    #[translate(ko = "NFT", en = "NFT")]
    Nft = 17,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum WaysToSell {
    #[default]
    Purchase = 1,
    Offer = 2,
    Bid = 3,
    Directly = 4,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ArtStyle {
    #[default]
    #[translate(ko = "Realism", en = "Realism")]
    Realism = 1,
    #[translate(ko = "Impressionism", en = "Impressionism")]
    Impressionism = 2,
    #[translate(ko = "Expressionism", en = "Expressionism")]
    Expressionism = 3,
    #[translate(ko = "Cubism", en = "Cubism")]
    Cubism = 4,
    #[translate(ko = "Surrealism", en = "Surrealism")]
    Surrealism = 5,
    #[translate(ko = "Minimalism", en = "Minimalism")]
    Minimalism = 6,
    #[translate(ko = "Pop Art", en = "Pop Art")]
    PopArt = 7,
    #[translate(ko = "Street Art", en = "Street Art")]
    StreetArt = 8,
    #[translate(ko = "Digital Art", en = "Digital Art")]
    DigitalArt = 9,
    #[translate(ko = "Photorealism", en = "Photorealism")]
    Photorealism = 10,
    #[translate(ko = "Collage", en = "Collage")]
    Collage = 11,
    #[translate(ko = "Conceptual Art", en = "Conceptual Art")]
    ConceptualArt = 12,
    #[translate(ko = "Futurism", en = "Futurism")]
    Futurism = 13,
    #[translate(ko = "Graffiti", en = "Graffiti")]
    Graffiti = 14,
    #[translate(ko = "Lowbrow", en = "Lowbrow")]
    Lowbrow = 15,
    #[translate(ko = "Neo-Expressionism", en = "Neo-Expressionism")]
    NeoExpressionism = 16,
    #[translate(ko = "Anime", en = "Anime")]
    Anime = 17,
    #[translate(ko = "Collectibles", en = "Collectibles")]
    Collectibles = 18,
    #[translate(ko = "Cartoon", en = "Cartoon")]
    Cartoon = 19,
    #[translate(ko = "Utility", en = "Utility")]
    Utility = 20,
    #[translate(ko = "Comic", en = "Comic")]
    Comic = 21,
    #[translate(ko = "Fantasy", en = "Fantasy")]
    Fantasy = 22,
    #[translate(ko = "Pixel Art", en = "Pixel Art")]
    PixelArt = 23,
    #[translate(ko = "Photography", en = "Photography")]
    Photography = 24,
    #[translate(ko = "Virtual Worlds", en = "Virtual Worlds")]
    VirtualWorlds = 25,
    #[translate(ko = "3D", en = "3D")]
    ThreeD = 26,
    #[translate(ko = "Illustration", en = "Illustration")]
    Illustration = 27,
    #[translate(ko = "Installation Art", en = "Installation Art")]
    InstallationArt = 28,
    #[translate(ko = "Performance Art", en = "Performance Art")]
    PerformanceArt = 29,
    #[translate(ko = "Typography", en = "Typography")]
    Typography = 30,
    #[translate(ko = "Still Life", en = "Still Life")]
    StillLife = 31,
    #[translate(ko = "Modernism", en = "Modernism")]
    Modernism = 32,
    #[translate(ko = "Classical", en = "Classical")]
    Classical = 33,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]

pub enum Material {
    #[default]
    #[translate(ko = "Acrylic", en = "Acrylic")]
    Acrylic = 1,
    #[translate(ko = "Aluminum", en = "Aluminum")]
    Aluminum = 2,
    #[translate(ko = "Aquatint", en = "Aquatint")]
    Aquatint = 3,
    #[translate(ko = "Arches Paper", en = "Arches Paper")]
    ArchesPaper = 4, // Rust 식별자에 맞게 변경
    #[translate(ko = "Art Paper", en = "Art Paper")]
    ArtPaper = 5, // Rust 식별자에 맞게 변경
    #[translate(ko = "Blown Glass", en = "Blown Glass")]
    BlownGlass = 6, // Rust 식별자에 맞게 변경
    #[translate(ko = "Brass", en = "Brass")]
    Brass = 7,
    #[translate(ko = "Bronze", en = "Bronze")]
    Bronze = 8,
    #[translate(ko = "C-Print", en = "C-Print")]
    CPrint = 9, // Rust 식별자에 맞게 변경
    #[translate(ko = "Canvas", en = "Canvas")]
    Canvas = 10,
    #[translate(ko = "Cardboard", en = "Cardboard")]
    Cardboard = 11,
    #[translate(ko = "Ceramic", en = "Ceramic")]
    Ceramic = 12,
    #[translate(ko = "Copper", en = "Copper")]
    Copper = 13,
    #[translate(ko = "Clay", en = "Clay")]
    Clay = 14,
    #[translate(ko = "Chalk", en = "Chalk")]
    Chalk = 15,
    #[translate(ko = "Charcoal", en = "Charcoal")]
    Charcoal = 16,
    #[translate(ko = "Collage", en = "Collage")]
    Collage = 17,
    #[translate(ko = "Concrete", en = "Concrete")]
    Concrete = 18,
    #[translate(ko = "Cotton", en = "Cotton")]
    Cotton = 19,
    #[translate(ko = "Crystal", en = "Crystal")]
    Crystal = 20,
    #[translate(ko = "Digital", en = "Digital")]
    Digital = 21,
    #[translate(ko = "Dry Point", en = "Dry Point")]
    DryPoint = 22, // Rust 식별자에 맞게 변경
    #[translate(ko = "Dye", en = "Dye")]
    Dye = 23,
    #[translate(ko = "Earthenware", en = "Earthenware")]
    Earthenware = 24,
    #[translate(ko = "Embroidery", en = "Embroidery")]
    Embroidery = 25,
    #[translate(ko = "Enamel", en = "Enamel")]
    Enamel = 26,
    #[translate(ko = "Engraving", en = "Engraving")]
    Engraving = 27,
    #[translate(ko = "Epoxy", en = "Epoxy")]
    Epoxy = 28,
    #[translate(ko = "Etching", en = "Etching")]
    Etching = 29,
    #[translate(ko = "Foam", en = "Foam")]
    Foam = 30,
    #[translate(ko = "Giclée", en = "Giclée")]
    Giclee = 31, // 특수 문자 제거
    #[translate(ko = "Glass", en = "Glass")]
    Glass = 32,
    #[translate(ko = "Glaze", en = "Glaze")]
    Glaze = 33,
    #[translate(ko = "Gold", en = "Gold")]
    Gold = 34,
    #[translate(ko = "Gouache", en = "Gouache")]
    Gouache = 35,
    #[translate(ko = "Ink", en = "Ink")]
    Ink = 36,
    #[translate(ko = "Inkjet Print", en = "Inkjet Print")]
    InkjetPrint = 37, // Rust 식별자에 맞게 변경
    #[translate(ko = "Iron", en = "Iron")]
    Iron = 38,
    #[translate(ko = "Lacquer", en = "Lacquer")]
    Lacquer = 39,
    #[translate(ko = "Leaf", en = "Leaf")]
    Leaf = 40,
    #[translate(ko = "Leather", en = "Leather")]
    Leather = 41,
    #[translate(ko = "Linen", en = "Linen")]
    Linen = 42,
    #[translate(ko = "Linocut", en = "Linocut")]
    Linocut = 43,
    #[translate(ko = "Lithograph", en = "Lithograph")]
    Lithograph = 44,
    #[translate(ko = "Mahogany", en = "Mahogany")]
    Mahogany = 45,
    #[translate(ko = "Marble", en = "Marble")]
    Marble = 46,
    #[translate(ko = "Metal", en = "Metal")]
    Metal = 47,
    #[translate(ko = "Mixed Media", en = "Mixed Media")]
    MixedMedia = 48, // Rust 식별자에 맞게 변경
    #[translate(ko = "Monoprint", en = "Monoprint")]
    Monoprint = 49,
    #[translate(ko = "Monotype", en = "Monotype")]
    Monotype = 50,
    #[translate(ko = "Oak", en = "Oak")]
    Oak = 51,
    #[translate(ko = "Oil", en = "Oil")]
    Oil = 52,
    #[translate(ko = "Paint", en = "Paint")]
    Paint = 53,
    #[translate(ko = "Panel", en = "Panel")]
    Panel = 54,
    #[translate(ko = "Paper", en = "Paper")]
    Paper = 55,
    #[translate(ko = "Pastel", en = "Pastel")]
    Pastel = 56,
    #[translate(ko = "Patina", en = "Patina")]
    Patina = 57,
    #[translate(ko = "Pencil", en = "Pencil")]
    Pencil = 58,
    #[translate(ko = "Pigment", en = "Pigment")]
    Pigment = 59,
    #[translate(ko = "Plaster", en = "Plaster")]
    Plaster = 60,
    #[translate(ko = "Plastic", en = "Plastic")]
    Plastic = 61,
    #[translate(ko = "Platinum", en = "Platinum")]
    Platinum = 62,
    #[translate(ko = "Plexiglass", en = "Plexiglass")]
    Plexiglass = 63,
    #[translate(ko = "Polaroid", en = "Polaroid")]
    Polaroid = 64,
    #[translate(ko = "Polyurethane", en = "Polyurethane")]
    Polyurethane = 65,
    #[translate(ko = "Porcelain", en = "Porcelain")]
    Porcelain = 66,
    #[translate(ko = "Powder", en = "Powder")]
    Powder = 67,
    #[translate(ko = "Rag Paper", en = "Rag Paper")]
    RagPaper = 68, // Rust 식별자에 맞게 변경
    #[translate(ko = "Resin", en = "Resin")]
    Resin = 69,
    #[translate(ko = "Screen Print", en = "Screen Print")]
    ScreenPrint = 70, // Rust 식별자에 맞게 변경
    #[translate(ko = "Silk", en = "Silk")]
    Silk = 71,
    #[translate(ko = "Silver", en = "Silver")]
    Silver = 72,
    #[translate(ko = "Silver Gelatin", en = "Silver Gelatin")]
    SilverGelatin = 73, // Rust 식별자에 맞게 변경
    #[translate(ko = "Sound", en = "Sound")]
    Sound = 74,
    #[translate(ko = "Spray Paint", en = "Spray Paint")]
    SprayPaint = 75, // Rust 식별자에 맞게 변경
    #[translate(ko = "Stainless Steel", en = "Stainless Steel")]
    StainlessSteel = 76, // Rust 식별자에 맞게 변경
    #[translate(ko = "Steel", en = "Steel")]
    Steel = 77,
    #[translate(ko = "Stone", en = "Stone")]
    Stone = 78,
    #[translate(ko = "Stoneware", en = "Stoneware")]
    Stoneware = 79,
    #[translate(ko = "Teak", en = "Teak")]
    Teak = 80,
    #[translate(ko = "Thread", en = "Thread")]
    Thread = 81,
    #[translate(ko = "Upholstery", en = "Upholstery")]
    Upholstery = 82,
    #[translate(ko = "Video", en = "Video")]
    Video = 83,
    #[translate(ko = "Vinyl", en = "Vinyl")]
    Vinyl = 84,
    #[translate(ko = "Walnut", en = "Walnut")]
    Walnut = 85,
    #[translate(ko = "Wash", en = "Wash")]
    Wash = 86,
    #[translate(ko = "Watercolor", en = "Watercolor")]
    Watercolor = 87,
    #[translate(ko = "Wire", en = "Wire")]
    Wire = 88,
    #[translate(ko = "Wood", en = "Wood")]
    Wood = 89,
    #[translate(ko = "Woodcut", en = "Woodcut")]
    Woodcut = 90,
    #[translate(ko = "Wool", en = "Wool")]
    Wool = 91,
    #[translate(ko = "Wove Paper", en = "Wove Paper")]
    WovePaper = 92, // Rust 식별자에 맞게 변경
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Size {
    #[default]
    None,
    Cm {
        w: f64,
        h: f64,
        d: f64,
    },
}

impl core::cmp::Eq for Size {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Weight {
    #[default]
    None,
    Kg(f64),
}

impl core::cmp::Eq for Weight {}
