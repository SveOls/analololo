use std::{error::Error, fs::File, io::Read};

use jomini::{text::ArrayReader, text::ObjectReader, Scalar, TextTape, Utf8Encoding};

mod ai;
mod battle_manager;
mod building_manager;
mod casualties;
mod character_manager;
mod civil_war;
mod combat_unit_manager;
mod counters;
mod country_formations;
mod country_manager;
mod country_rankings;
mod cultures;
mod decree_manager;
mod diplomatic_actions;
mod diplomatic_plays;
mod election_manager;
mod events;
mod fronts;
mod game_rules;
mod hq_manager;
mod institutions;
mod interest_groups;
mod interest_manager;
mod ironman;
mod journal_entry_manager;
pub mod laws;
mod market_manager;
mod metadata;
mod naval_invasions;
mod objective_manager;
mod order_manager;
mod pacts;
mod parties;
mod political_movement_manager;
mod pops;
mod previously_played;
mod proposals;
mod provinces;
mod relations;
mod shipping_lane_manager;
mod state_region_manager;
mod states;
mod supply_area_manager;
mod supply_network_manager;
mod tasks;
mod technology;
mod theaters;
mod trade_route_manager;
mod tutorial_manager;
mod variables;
mod war_manager;
use ai::Ai;
use battle_manager::BattleManager;
use building_manager::BuildingManager;
use casualties::Casualties;
use character_manager::CharacterManager;
use civil_war::CivilWar;
use combat_unit_manager::CombatUnitManager;
use counters::Counters;
use country_formations::CountryFormations;
use country_manager::CountryManager;
use country_rankings::CountryRankings;
use cultures::Cultures;
use decree_manager::DecreeManager;
use diplomatic_actions::DiplomaticActions;
use diplomatic_plays::DiplomaticPlays;
use election_manager::ElectionManager;
use events::Events;
use fronts::Fronts;
use game_rules::GameRules;
use hq_manager::HqManager;
use institutions::Institutions;
use interest_groups::InterestGroups;
use interest_manager::InterestManager;
use ironman::Ironman;
use journal_entry_manager::JournalEntryManager;
use laws::Laws;
use market_manager::MarketManager;
use metadata::Metadata;
use naval_invasions::NavalInvasions;
use objective_manager::ObjectiveManager;
use order_manager::OrderManager;
use pacts::Pacts;
use parties::Parties;
use political_movement_manager::PoliticalMovementManager;
use pops::Pops;
use previously_played::PreviouslyPlayed;
use proposals::Proposals;
use provinces::Province;
use relations::Relations;
use shipping_lane_manager::ShippingLaneManager;
use state_region_manager::StateRegionManager;
use states::States;
use supply_area_manager::SupplyAreaManager;
use supply_network_manager::SupplyNetworkManager;
use tasks::Tasks;
use technology::Technology;
use theaters::Theaters;
use trade_route_manager::TradeRouteManager;
use tutorial_manager::TutorialManager;
use variables::Variables;
use war_manager::WarManager;
use zip::ZipArchive;

#[allow(dead_code)]
pub struct Save {
    metadata: Metadata,
    ironman: Ironman,
    pops: Pops,
    provinces: Vec<Province>,
    states: States,
    cultures: Cultures,
    playthrough_id: String,
    character_manger: CharacterManager,
    seed: i64,
    pacts: Pacts,
    tutorial_manager: TutorialManager,
    battle_manager: BattleManager,
    technology: Technology,
    seed_count: i64,
    previously_played: Vec<PreviouslyPlayed>,
    speed: i64,
    tasks: Tasks,
    supply_network_manager: SupplyNetworkManager,
    relations: Relations,
    ai: Ai,
    first_start: bool,
    game_rules: GameRules,
    variables: Variables,
    war_manager: WarManager,
    events: Events,
    counters: Counters,
    institutions: Institutions,
    diplomatic_plays: DiplomaticPlays,
    diplomatic_actions: DiplomaticActions,
    market_manager: MarketManager,
    parties: Parties,
    country_formations: CountryFormations,
    country_rankings: CountryRankings,
    laws: Laws,
    country_manager: CountryManager,
    interest_groups: InterestGroups,
    shipping_lane_manager: ShippingLaneManager,
    proposals: Proposals,
    theaters: Theaters,
    casualties: Casualties,
    journal_entry_manager: JournalEntryManager,
    building_manager: BuildingManager,
    date: String,
    supply_area_manager: SupplyAreaManager,
    naval_invasions: NavalInvasions,
    civil_war: CivilWar,
    order_manager: OrderManager,
    fronts: Fronts,
    decree_manager: DecreeManager,
    election_manager: ElectionManager,
    combat_unit_manager: CombatUnitManager,
    hq_manager: HqManager,
    objective_manager: ObjectiveManager,
    interest_manager: InterestManager,
    state_region_manager: StateRegionManager,
    trade_route_manager: TradeRouteManager,
    political_movement_manager: PoliticalMovementManager,
}

impl Save {
    pub fn states(&self) -> &States {
        &self.states
    }
    pub fn laws(&self) -> &Laws {
        &self.laws
    }
    pub fn trade_routes(&self) -> &TradeRouteManager {
        &self.trade_route_manager
    }
    pub fn pops(&self) -> &Pops {
        &self.pops
    }
    pub fn countries(&self) -> &CountryManager {
        &self.country_manager
    }
    pub fn cultures(&self) -> &Cultures {
        &self.cultures
    }
    pub fn buildings(&self) -> &BuildingManager {
        &self.building_manager
    }
    pub fn new(stuff: File) -> Result<Self, Box<dyn Error>> {
        let mut a = ZipArchive::new(stuff)?;
        let mut info = Vec::new();
        a.by_name("gamestate")?.read_to_end(&mut info)?;
        let inp = TextTape::from_slice(&info)?;
        let inp = inp.utf8_reader();

        let mut metadata = None;
        let mut ironman = None;
        let mut country_manager = None;
        let mut provinces = None;
        let mut pops = None;
        let mut states = None;
        let mut country_rankings = None;
        let mut cultures = None;
        let mut playthrough_id = None;
        let mut relations = None;
        let mut seed = None;
        let mut battle_manager = None;
        let mut ai = None;
        let mut tasks = None;
        let mut speed = None;
        let mut country_formations = None;
        let mut diplomatic_actions = None;
        let mut previously_played = None;
        let mut interest_groups = None;
        let mut tutorial_manager = None;
        let mut supply_area_manager = None;
        let mut market_manager = None;
        let mut first_start = None;
        let mut seed_count = None;
        let mut war_manager = None;
        let mut variables = None;
        let mut laws = None;
        let mut parties = None;
        let mut proposals = None;
        let mut game_rules = None;
        let mut theaters = None;
        let mut diplomatic_plays = None;
        let mut technology = None;
        let mut counters = None;
        let mut character_manager = None;
        let mut pacts = None;
        let mut events = None;
        let mut institutions = None;
        let mut journal_entry_manager = None;
        let mut casualties = None;
        let mut shipping_lane_manager = None;
        let mut building_manager = None;
        let mut naval_invasions = None;
        let mut civil_war = None;
        let mut order_manager = None;
        let mut fronts = None;
        let mut decree_manager = None;
        let mut election_manager = None;
        let mut combat_unit_manager = None;
        let mut hq_manager = None;
        let mut objective_manager = None;
        let mut interest_manager = None;
        let mut supply_network_manager = None;
        let mut state_region_manager = None;
        let mut trade_route_manager = None;
        let mut date = None;
        let mut political_movement_manager = None;

        for (key, _, value) in inp.fields() {
            // println!("start {}", key.read_string());
            match key.read_str().as_ref() {
                "meta_data" => metadata = Some(Metadata::new(value.read_object()?)?),
                "ironman" => ironman = Some(Ironman::new(value.read_object()?)?),
                "playthrough_id" => playthrough_id = Some(value.read_string()?),
                "date" => date = Some(value.read_string()?),
                "pops" => pops = Some(Pops::new(value.read_object()?)?),
                "seed" => seed = Some(value.read_scalar()?.to_i64()?),
                "seed_count" => seed_count = Some(value.read_scalar()?.to_i64()?),
                "speed" => speed = Some(value.read_scalar()?.to_i64()?),
                "first_start" => first_start = Some(value.read_scalar()?.to_bool()?),
                "previous_played" => {
                    previously_played = Some(PreviouslyPlayed::new_group(value.read_array()?)?)
                }
                "counters" => counters = Some(Counters::new(value.read_object()?)?),
                "variables" => variables = Some(Variables::new(value.read_object()?)?),
                "provinces" => provinces = Some(Province::new_group(value.read_object()?)?),
                "country_manager" => {
                    country_manager = Some(CountryManager::new(value.read_object()?)?)
                }
                "states" => states = Some(States::new(value.read_object()?)?),
                "interest_groups" => {
                    interest_groups = Some(InterestGroups::new(value.read_object()?)?)
                }
                "country_rankings" => {
                    country_rankings = Some(CountryRankings::new(value.read_object()?)?)
                }
                "parties" => parties = Some(Parties::new(value.read_object()?)?),
                "laws" => laws = Some(Laws::new(value.read_object()?)?),
                "institutions" => institutions = Some(Institutions::new(value.read_object()?)?),
                "cultures" => cultures = Some(Cultures::new(value.read_object()?)?),
                "character_manager" => {
                    character_manager = Some(CharacterManager::new(value.read_object()?)?)
                }
                "market_manager" => {
                    market_manager = Some(MarketManager::new(value.read_object()?)?)
                }
                "technology" => technology = Some(Technology::new(value.read_object()?)?),
                "events" => events = Some(Events::new(value.read_object()?)?),
                "pacts" => pacts = Some(Pacts::new(value.read_object()?)?),
                "relations" => relations = Some(Relations::new(value.read_object()?)?),
                "diplomatic_plays" => {
                    diplomatic_plays = Some(DiplomaticPlays::new(value.read_object()?)?)
                }
                "diplomatic_actions" => {
                    diplomatic_actions = Some(DiplomaticActions::new(value.read_object()?)?)
                }
                "ai" => ai = Some(Ai::new(value.read_object()?)?),
                "war_manager" => war_manager = Some(WarManager::new(value.read_object()?)?),
                "battle_manager" => {
                    battle_manager = Some(BattleManager::new(value.read_object()?)?)
                }
                "proposals" => proposals = Some(Proposals::new(value.read_object()?)?),
                "theaters" => theaters = Some(Theaters::new(value.read_object()?)?),
                "building_manager" => {
                    building_manager = Some(BuildingManager::new(value.read_object()?)?)
                }
                "trade_route_manager" => {
                    trade_route_manager = Some(TradeRouteManager::new(value.read_object()?)?)
                }
                "decree_manager" => {
                    decree_manager = Some(DecreeManager::new(value.read_object()?)?)
                }
                "fronts" => fronts = Some(Fronts::new(value.read_object()?)?),
                "interest_manager" => {
                    interest_manager = Some(InterestManager::new(value.read_object()?)?)
                }
                "order_manager" => order_manager = Some(OrderManager::new(value.read_object()?)?),
                "naval_invasions" => {
                    naval_invasions = Some(NavalInvasions::new(value.read_object()?)?)
                }
                "civil_war" => civil_war = Some(CivilWar::new(value.read_object()?)?),
                "combat_unit_manager" => {
                    combat_unit_manager = Some(CombatUnitManager::new(value.read_object()?)?)
                }
                "journal_entry_manager" => {
                    journal_entry_manager = Some(JournalEntryManager::new(value.read_object()?)?)
                }
                "state_region_manager" => {
                    state_region_manager = Some(StateRegionManager::new(value.read_object()?)?)
                }
                "election_manager" => {
                    election_manager = Some(ElectionManager::new(value.read_object()?)?)
                }
                "hq_manager" => hq_manager = Some(HqManager::new(value.read_object()?)?),
                "objective_manager" => {
                    objective_manager = Some(ObjectiveManager::new(value.read_object()?)?)
                }
                "political_movement_manager" => {
                    political_movement_manager =
                        Some(PoliticalMovementManager::new(value.read_object()?)?)
                }
                "casualties" => casualties = Some(Casualties::new(value.read_object()?)?),
                "shipping_lane_manager" => {
                    shipping_lane_manager = Some(ShippingLaneManager::new(value.read_object()?)?)
                }
                "tasks" => tasks = Some(Tasks::new(value.read_object()?)?),
                "game_rules" => game_rules = Some(GameRules::new(value.read_object()?)?),
                "supply_network_manager" => {
                    supply_network_manager = Some(SupplyNetworkManager::new(value.read_object()?)?)
                }
                "supply_area_manager" => {
                    supply_area_manager = Some(SupplyAreaManager::new(value.read_object()?)?)
                }
                "country_formations" => {
                    country_formations = Some(CountryFormations::new(value.read_object()?)?)
                }
                "tutorial_manager" => {
                    tutorial_manager = Some(TutorialManager::new(value.read_object()?)?)
                }
                a => println!("missing >{a}<"),
            }
            // println!("end {}", key.read_string());
        }
        Ok(Self {
            metadata: metadata.unwrap(),
            character_manger: character_manager.unwrap(),
            ironman: ironman.unwrap(),
            provinces: provinces.unwrap(),
            pops: pops.unwrap(),
            country_formations: country_formations.unwrap(),
            cultures: cultures.unwrap(),
            technology: technology.unwrap(),
            tutorial_manager: tutorial_manager.unwrap(),
            journal_entry_manager: journal_entry_manager.unwrap(),
            casualties: casualties.unwrap(),
            diplomatic_plays: diplomatic_plays.unwrap(),
            states: states.unwrap(),
            supply_area_manager: supply_area_manager.unwrap(),
            battle_manager: battle_manager.unwrap(),
            ai: ai.unwrap(),
            relations: relations.unwrap(),
            events: events.unwrap(),
            tasks: tasks.unwrap(),
            date: date.unwrap(),
            diplomatic_actions: diplomatic_actions.unwrap(),
            pacts: pacts.unwrap(),
            playthrough_id: playthrough_id.unwrap(),
            supply_network_manager: supply_network_manager.unwrap(),
            seed: seed.unwrap(),
            speed: speed.unwrap(),
            country_rankings: country_rankings.unwrap(),
            variables: variables.unwrap(),
            war_manager: war_manager.unwrap(),
            laws: laws.unwrap(),
            market_manager: market_manager.unwrap(),
            country_manager: country_manager.unwrap(),
            parties: parties.unwrap(),
            interest_groups: interest_groups.unwrap(),
            seed_count: seed_count.unwrap(),
            first_start: first_start.unwrap(),
            institutions: institutions.unwrap(),
            counters: counters.unwrap(),
            shipping_lane_manager: shipping_lane_manager.unwrap(),
            proposals: proposals.unwrap(),
            theaters: theaters.unwrap(),
            building_manager: building_manager.unwrap(),
            naval_invasions: naval_invasions.unwrap(),
            civil_war: civil_war.unwrap(),
            order_manager: order_manager.unwrap(),
            fronts: fronts.unwrap(),
            decree_manager: decree_manager.unwrap(),
            game_rules: game_rules.unwrap(),
            election_manager: election_manager.unwrap(),
            combat_unit_manager: combat_unit_manager.unwrap(),
            hq_manager: hq_manager.unwrap(),
            objective_manager: objective_manager.unwrap(),
            interest_manager: interest_manager.unwrap(),
            state_region_manager: state_region_manager.unwrap(),
            trade_route_manager: trade_route_manager.unwrap(),
            political_movement_manager: political_movement_manager.unwrap(),
            previously_played: previously_played.unwrap_or(Vec::new()),
        })
    }
}
