use crate::routes::get_armor::get_armor;
use axum::{
    routing::{get, post},
    Router,
};
use data_model::{
    armor_model_from_armor_schema, deserialize_equipment, read_equipment_file,
    weapon_model_from_weapon_schema,
};
use routes::{get_constants::get_constants, get_weapons::get_weapons, post_attack::post_attack};
use rs5e_concepts::{
    ability_scores::AbilityScores, armor::ArmorModel, armor_type::ArmorType, class_type::ClassType,
    cover_state::CoverState, hp::Hp, id::Id, level::Level, prone_state::ProneState,
    weapon::WeaponModel, weapon_type::WeaponType,
};
use rs5e_entities::{armor::ArmorEntity, character::CharacterEntity, weapon::WeaponEntity};
use rs5e_schema::{armor::ArmorSchema, weapon::WeaponSchema};
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    sync::Arc,
};
use tower_http::services::{ServeDir, ServeFile};
use typeshare::typeshare;

mod data_model;
mod routes;

pub(crate) const STATIC_CLIENT_DIR: &str = "client/dist";

// the application state
//
// here you can put configuration, database connection pools, or whatever
// state you need
//
// see "When states need to implement `Clone`" for more details on why we need
// `#[derive(Clone)]` here.
#[derive(Clone)]
pub(crate) struct AppState {
    pub weapon_model_map: Arc<HashMap<WeaponType, WeaponModel>>,
    pub armor_model_map: Arc<HashMap<ArmorType, ArmorModel>>,
}

#[typeshare]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CharacterBuilder {
    name: String,
    level: u32,
    hp: u32,
    class: ClassType,
    ability_scores: AbilityScores,
    weapon_type: Option<WeaponType>,
    armor_type: Option<ArmorType>,
    // These are not properties of a unit but rather of circumstance
    prone_state: ProneState,
    cover_state: CoverState,
}

fn character_from_builder<'a>(
    builder: CharacterBuilder,
    weapon_model_map: &'a HashMap<WeaponType, WeaponModel>,
    armor_model_map: &'a HashMap<ArmorType, ArmorModel>,
) -> CharacterEntity<'a> {
    let weapon = builder
        .weapon_type
        .as_ref()
        .map(|weapon_type| WeaponEntity {
            id: Id::new_incremental(),
            model: weapon_model_map.get(weapon_type).unwrap(),
        });

    let armor = builder.armor_type.as_ref().map(|armor_type| ArmorEntity {
        id: Id::new_incremental(),
        model: armor_model_map.get(armor_type).unwrap(),
    });

    CharacterEntity {
        id: Id::new_incremental(),
        name: builder.name.to_string(),
        hp: Hp::new(builder.hp),
        equipped_weapon: weapon,
        equipped_armor: armor,
        ability_scores: builder.ability_scores,
        class: builder.class,
        level: Level::try_from(builder.level).unwrap(),
        prone_state: builder.prone_state,
        cover_state: builder.cover_state,
    }
}

async fn rs5e_server(ip: impl Into<IpAddr>, port: u16) {
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let addr = SocketAddr::from((ip, port));
    dbg!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(using_serve_dir_with_assets_fallback().into_make_service())
        .await
        .unwrap();
}

fn using_serve_dir_with_assets_fallback() -> Router {
    // `ServeDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/doesnt-exist.jpg` will return `index.html`
    // rather than a 404
    let static_dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(STATIC_CLIENT_DIR);
    let serve_dir =
        ServeDir::new(static_dir_path).not_found_service(ServeFile::new("assets/index.html"));

    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/src");

    let equipment_file_string: String = read_equipment_file(data_dir);

    let equipment = deserialize_equipment(&equipment_file_string);

    let weapon_model_map = equipment
        .iter()
        .filter_map(|equip| WeaponSchema::try_from(equip).ok())
        .map(|weapon_schema| {
            let model = weapon_model_from_weapon_schema(&weapon_schema);
            (model.weapon_type.clone(), model)
        })
        .collect::<HashMap<_, _>>();

    let armor_model_map = equipment
        .iter()
        .filter_map(|equip| ArmorSchema::try_from(equip).ok())
        .map(|armor_schema| {
            let model = armor_model_from_armor_schema(&armor_schema);
            (model.armor_type.clone(), model)
        })
        .collect::<HashMap<_, _>>();

    let state = AppState {
        weapon_model_map: Arc::new(weapon_model_map),
        armor_model_map: Arc::new(armor_model_map),
    };

    Router::new()
        .route("/test", get(|| async { "hi from test" }))
        .route("/attack", post(post_attack))
        .route("/get-weapons", get(get_weapons))
        .route("/get-armor", get(get_armor))
        .route("/get-constants", get(get_constants))
        .nest_service("/", serve_dir)
        .with_state(state)
    // .fallback_service(serve_dir)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (addr, port) = match args.get(1) {
        Some(ip_string) => {
            let mut ip_and_port = ip_string.split(':');

            let ip = ip_and_port.next().expect("IP address not provided");
            let port = ip_and_port
                .next()
                .expect("Port not provided")
                .parse::<u16>()
                .expect("Port is not a number");

            let addr: Ipv4Addr = ip.parse().expect("IP parse failed");

            (addr, port)
        }
        None => (Ipv4Addr::LOCALHOST, 8080),
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            rs5e_server(addr, port).await;
        })
}
