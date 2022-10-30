use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct SpawnPoint {
    pub x: f32,
    pub y: f32,
}

const SPAWN_POINT_N1: SpawnPoint = SpawnPoint { x: 13.5, y: 1.5 };
const SPAWN_POINT_N2: SpawnPoint = SpawnPoint { x: 17.5, y: 1.5 };
const SPAWN_POINT_N3: SpawnPoint = SpawnPoint { x: 21.5, y: 1.5 };
const SPAWN_POINT_N4: SpawnPoint = SpawnPoint { x: 25.5, y: 1.5 };
const SPAWN_POINT_N4_WALL: SpawnPoint = SpawnPoint { x: 25.5, y: 3.5 };

const SPAWN_POINT_S1: SpawnPoint = SpawnPoint { x: 13.5, y: 21.5 };
const SPAWN_POINT_S2: SpawnPoint = SpawnPoint { x: 17.5, y: 21.5 };
const SPAWN_POINT_S3: SpawnPoint = SpawnPoint { x: 21.5, y: 21.5 };
const SPAWN_POINT_S4: SpawnPoint = SpawnPoint { x: 25.5, y: 21.5 };
const SPAWN_POINT_S4_WALL: SpawnPoint = SpawnPoint { x: 25.5, y: 19.5 };

const SPAWN_POINTS: &[SpawnPoint] = &[
    SPAWN_POINT_N1,
    SPAWN_POINT_N2,
    SPAWN_POINT_N3,
    SPAWN_POINT_N4,
    SPAWN_POINT_N4_WALL,
    SPAWN_POINT_S1,
    SPAWN_POINT_S2,
    SPAWN_POINT_S3,
    SPAWN_POINT_S4,
    SPAWN_POINT_S4_WALL,
];

const MAX_SPAWNS: usize = SPAWN_POINTS.len();

pub fn generate_spawn_points(k: usize) -> Vec<SpawnPoint> {
    let mut rng = rand::thread_rng();

    let mut spawns = vec![];

    for i in 0..MAX_SPAWNS {
        let rn = MAX_SPAWNS - i;
        let rk = k - spawns.len();

        if rng.gen_range(0..=MAX_SPAWNS) % rn < rk {
            spawns.push(SPAWN_POINTS[i]);
            if spawns.len() >= k {
                break;
            }
        }
    }

    spawns
}
