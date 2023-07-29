import { fetchDriverStandings } from "../fetchers/fetchDriverStandings";
import { useQuery } from '@tanstack/react-query';
import DriverBlock from "./DriverBlock";
import TopDriverBlock from "./TopDriverBlock";
import { DriverTypes } from "../interfaces/interfaces";

export default function DriverList() {
    const { data, isLoading, isError, isSuccess } = useQuery<any>({ queryKey: ['driverStandings'], queryFn: fetchDriverStandings });

    if (isLoading) {
        return <div>Loading...</div>;
    }

    if (isError) {
        return <div>Error occurred while fetching data.</div>;
    }

    const TopDriverLoop = () => {
        if (data) {
            console.log(data);
            return (
                <>
                    <div className="flex flex-row justify-center gap-5">
                        {data && data
                            .slice(1, 2)
                            .map((data: DriverTypes) =>
                                <TopDriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} driverRef={data.driver_ref} />
                            )}
                        {data && data
                            .slice(0, 1)
                            .map((data: DriverTypes) =>
                                <TopDriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} driverRef={data.driver_ref} />
                            )}
                        {data && data
                            .slice(2, 3)
                            .map((data: DriverTypes) =>
                                <TopDriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} driverRef={data.driver_ref} />
                            )}
                    </div>
                </>
            )
        }
    }

    const DriverLoop = () => {
        if (data) {
            return (
                <div>
                    {data &&
                        data
                            .slice(3, 20)
                            .map((data: DriverTypes) =>
                                <DriverBlock key={data.driver_id} countryCode={data.country_code} forename={data.forename} surename={data.surename} points={data.points} position={data.position} driverId={data.driver_id} />
                            )}
                </div>
            )
        }
    }

    if (isSuccess) {
        return (
            <>
                <TopDriverLoop />
                <DriverLoop />
            </>
        )
    }
}