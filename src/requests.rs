// Constants to represent the various API endpoints
// Game status, register agent
const GAME_STATUS: &str = "https://api.spacetraders.io/v2/";
const REGISTER: &str = "https://api.spacetraders.io/v2/register";
// Agents
const GET_AGENT: &str = "https://api.spacetraders.io/v2/my/agent";
const LIST_AGENTS: &str = "https://api.spacetraders.io/v2/agents";
const GET_PUBLIC_AGENT: &str = "https://api.spacetraders.io/v2/agents/{agentSymbol}";
// Contracts
const LIST_CONTRACTS: &str = "https://api.spacetraders.io/v2/my/contracts";
const GET_CONTRACT: &str = "https://api.spacetraders.io/v2/my/contracts/{contractId}";
const ACCEPT_CONTRACT: &str = "https://api.spacetraders.io/v2/my/contracts/{contractId}/accept";
const DELIVER_CARGO: &str = "https://api.spacetraders.io/v2/my/contracts/{contractId}/deliver";
const FULFILL_CONTRACT: &str = "https://api.spacetraders.io/v2/my/contracts/{contractId}/fulfill";
// Factions
const LIST_FACTIONS: &str = "https://api.spacetraders.io/v2/factions";
const GET_FACTION: &str = "https://api.spacetraders.io/v2/factions/{factionSymbol}";
// Fleet
const LIST_SHIPS: &str = "https://api.spacetraders.io/v2/my/ships";
const GET_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}";
const GET_SHIP_CARGO: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/cargo";
const ORBIT_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/orbit";
const SHIP_REFINE: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/refine";
const CREATE_CHART: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/chart";
const GET_SHIP_COOLDOWN: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/cooldown";
const DOCK_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/dock";
const CREATE_SURVEY: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/survey";
const EXTRACT_RESOURCES: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/extract";
const SIPHON_RESOURCES: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/siphon";
const EXTRACT_W_SURVEY: &str =
    "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/extract/survey";
const JETTISON_CARGO: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/jettison";
const JUMP_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/jump";
const NAVIGATE_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/navigate";
const PATCH_SHIP_NAV: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/nav";
const GET_SHIP_NAV: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/nav";
const SELL_CARGO: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/sell";
const SCAN_SYSTEMS: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/scan/systems";
const SCAN_WAYPOINTS: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/scan/waypoints";
const SCAN_SHIPS: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/scan/ships";
const REFUEL_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/refuel";
const PURCHASE_CARGO: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/purchase";
const TRANSFER_CARGO: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/transfer";
const NEGOTIATE_CONTRACT: &str =
    "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/negotiate/contract";
const GET_MOUNTS: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/mounts";
const INSTALL_MOUNT: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/mounts/install";
const REMOVE_MOUNT: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/mounts/remove";
const SCRAP_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/scrap";
const GET_REPAIR_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/repair";
const REPAIR_SHIP: &str = "https://api.spacetraders.io/v2/my/ships/{shipSymbol}/repair";
// Systems
const LIST_SYSTEMS: &str = "https://api.spacetraders.io/v2/systems";
const GET_SYSTEM: &str = "https://api.spacetraders.io/v2/systems/{systemSymbol}";
const LIST_WAYPOINTS: &str = "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints";
const GET_WAYPOINT: &str =
    "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints/{waypointSymbol}";
const GET_MARKET: &str =
    "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints/{waypointSymbol}/market";
const GET_SHIPYARD: &str =
    "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard";
const GET_JUMPGATE: &str =
    "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate";
const GET_CONSTRUCTION_SITE: &str =
    "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints/{waypointSymbol}/construction";
const SUPPLY_CONSTRUCTION_SITE: &str =
    "https://api.spacetraders.io/v2/systems/{systemSymbol}/waypoints/{waypointSymbol}/construction/supply";
