import NextEventDropDownBlock from "./NextEventDropDownBlock"
import { EventTypes } from "../interfaces/interfaces"

export default function NextEventDropDown({ nextEvent, title }: EventTypes) {

    return (

        <div className="relative z-20 flex flex-col w-full mt-8">
            <p className="dark:text-white text-black font-bold text-2xl mt-3 ml-4">{title}</p>
            <hr className="h-px my-2 ml-4 bg-neutral-200 border-0 dark:bg-neutral-900" />
            <NextEventDropDownBlock circuitName="Free Practice 1" date={nextEvent.fp1_date} time={nextEvent.fp1_time} />
            <NextEventDropDownBlock circuitName="Free Practice 2" date={nextEvent.fp2_date} time={nextEvent.fp2_time} />
            <NextEventDropDownBlock circuitName="Free Practice 3" date={nextEvent.fp3_date} time={nextEvent.fp3_time} />
            <NextEventDropDownBlock circuitName="Qualifying" date={nextEvent.quali_date} time={nextEvent.quali_time} />
            <NextEventDropDownBlock circuitName="Sprint" date={nextEvent.sprint_date} time={nextEvent.sprint_time} />
            <NextEventDropDownBlock circuitName={nextEvent.grand_prix_name} date={nextEvent.grand_prix_date} time={nextEvent.grand_prix_time} />
        </div>
    )
}