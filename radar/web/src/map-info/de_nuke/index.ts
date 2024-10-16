import { LoadedMap } from "..";
import SimpleRadarDefault from "./radar_0_default.png";
import SimpleRadarLower from "./radar_0_lower.png";
import OfficialDefault from "./radar_1_default.png";
import OfficialLower from "./radar_1_lower.png";

export default {
    mapName: "de_nuke",
    displayName: "Nuke",

    pos_x: -3453, // upper left world coordinate
    pos_y: 2887,
    scale: 7,

    verticalSections: {
        default: // use the primary radar image
        {
            altitudeMax: 10000,
            altitudeMin: -495,
        },
        lower: // i.e. radar_x_lower.png
        {
            altitudeMax: -495,
            altitudeMin: -10000,
        }
    },

    mapImages: [
        {
            name: "SimpleRadar",
            images:{
                default: SimpleRadarDefault,
                lower: SimpleRadarLower
            }
        },
        {
            name: "Official",
            images:{
                default: OfficialDefault,
                lower: OfficialLower
            }
        }
    ]
} satisfies LoadedMap;