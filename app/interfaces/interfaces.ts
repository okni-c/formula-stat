export interface ConstructorTypes {
    constructor_id: string,
    points: string,
    position: number,
    wins: string,
    name: string,
    nationality: string,
    country_code: string,
    constructor_ref: string,
}

export interface DriverTypes {
    driver_id: string,
    driver_ref: string,
    points: string,
    position: number,
    wins: string,
    forename: string,
    surename: string,
    nationality: string,
    country_code: string,
}

export interface HeaderTypes {
    round: number,
    circuitName: string,
    trackImg?: string,
    removeImg?: boolean,
}

export interface ListTypes {
    title: string,
    children: any,
}

export interface EventTypes {
    nextEvent: {
        next_event_name: string,
        next_event_time: string,
        grand_prix_name: string,
        grand_prix_date: string,
        grand_prix_time: string,
        fp1_date: string,
        fp1_time: string,
        fp2_date: string,
        fp2_time: string,
        fp3_date: string,
        fp3_time: string,
        quali_time: string,
        quali_date: string,
        sprint_date: string,
        sprint_time: string,
    }
    title?: string
}

export interface NextEventDropDownBlockTypes {
    circuitName: string,
    date: string,
    time: string,
}

export interface RaceBlockTypes {
    circuitName: string,
    winner?: string,
    date: string,
    flagcode?: string,
    time: string,
}

export interface ContainerTypes {
    title: string,
    children: any,
    dropDown?: boolean,
}

export interface YearlyRaceDataTypes {
    circuit_id: string,
    name: string,
    date: string,
    time: string,
    country_code: string,
}

export interface TopDriverBlockTypes {
    countryCode: string,
    forename: string,
    surename: string,
    points: string,
    position: number,
    driverId: string,
    driverRef: string,
  }