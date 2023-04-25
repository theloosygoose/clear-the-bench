export type GameSaveData = {
    save_name: string,
    id: number,
    user_team: string,
    year: number,
}

export type IntangibleRatings ={
    ball_handling: number,
    burst: number,
    def_awareness:number,
    fluidity: number,
    hands: number,
    height: number,
    off_awareness: number,
    pass_accuracy: number,
    shot_form: number,
    sliding: number,
    speed: number,
    strength: number,
    touch: number,
    wingspan: number,
}

export type TangibleRatings = {
    creation_off_ball: number,
    creation_on_ball:number,
    defense_block: number,
    defense_off_ball: number,
    defense_on_ball: number,
    defense_steal: number,
    foul_drawing: number,
    hustle: number,
    pass_creativity: number,
    pass_iq:number,
    pass_vision:number,
    rim_finishing:number,
    shot_close:number,
    shot_mid:number,
    shot_movement:number,
    shot_three:number,
}

export type Player = {
    ratings: IntangibleRatings,
    skills: TangibleRatings, 
}

export type Personality = {
    adaptability: number,
    creativity: number,
    dog_factor:number,
    intelligence: number,
    loyalty: number,
    work_ethic:number,
}

export type PlayerPerson = {
    age: number,
    country: string,
    job: { Player: Player },
    name: string,
    personality: Personality,
}


export type Team = {
    coach: string,
    losses: number,
    name: string,
    owner: string,
    players: Array<PlayerPerson>,
    team_salary: number,
    wins: number,
} 