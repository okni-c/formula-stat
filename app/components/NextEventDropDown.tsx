import NextEventDropDownBlock from "./NextEventDropDownBlock"

interface EventTypes {
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
    title: string,
}

export default function NextEventDropDown({ nextEvent, title }: EventTypes) {

    return (
        <div className="flex flex-col mt-8">
            <p className="text-white font-bold text-2xl mt-3 ml-4">{title}</p>
            <hr className="h-px my-2 ml-4 bg-neutral-200 border-0 dark:bg-neutral-800"/>
            <NextEventDropDownBlock circuitName="Free Practice 1" date={nextEvent.fp1_date} time={nextEvent.fp1_time} />
            <NextEventDropDownBlock circuitName="Free Practice 2" date={nextEvent.fp2_date} time={nextEvent.fp2_time} />
            <NextEventDropDownBlock circuitName="Free Practice 3" date={nextEvent.fp3_date} time={nextEvent.fp3_time} />
            <NextEventDropDownBlock circuitName="Qualifying" date={nextEvent.quali_date} time={nextEvent.quali_time} />
            <NextEventDropDownBlock circuitName="Sprint" date={nextEvent.sprint_date} time={nextEvent.sprint_time} />
            <NextEventDropDownBlock circuitName={nextEvent.grand_prix_name} date={nextEvent.sprint_date} time={nextEvent.sprint_time} />
        </div>
    )
}