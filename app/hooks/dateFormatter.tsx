/**
 * Proccess UTC Date to local time zone and format based on the option passed.
 * @param string date - UTC date from server
 * @param string time - UTC time from server
 * @param string option - 'day', 'time', or null for full date
 */

export function formatDate(dateString: string, timeString: string, option?: string | undefined): string {
    const dateTimeString = dateString + 'T' + timeString;
    const dateObject = new Date(dateTimeString);

    // Get the local time offset in minutes
    const localTimeOffsetMinutes = dateObject.getTimezoneOffset();

    // Adjust the date and time using the local time offset
    dateObject.setMinutes(dateObject.getMinutes() - localTimeOffsetMinutes);

    const dateOption: any = { month: 'long', day: 'numeric', year: 'numeric' };
    const timeOption: any = { hour: 'numeric', minute: 'numeric' };
    const dayOption: any = { weekday: 'long' };

    if (option === 'day') {
        return dateObject.toLocaleDateString(undefined, dayOption);
    }

    if (option === 'time') {
        return dateObject.toLocaleTimeString(undefined, timeOption);
    }

    else {
        return dateObject.toLocaleDateString(undefined, dateOption);
    }
}

export function formatDateTime(dateTimeString: string): string | undefined {
    if (dateTimeString) {
        // Split the dateTimeString into dateString and timeString
        const [datePart, timePart] = dateTimeString.split(' ');
        const dateObject = new Date(datePart + 'T' + timePart);

        // Get the local time offset in minutes
        const localTimeOffsetMinutes = dateObject.getTimezoneOffset();

        // Adjust the date and time using the local time offset
        dateObject.setMinutes(dateObject.getMinutes() - localTimeOffsetMinutes);

        // Format options for date and time
        const dateFormatOptions: any = { month: 'long', day: 'numeric', year: 'numeric' };
        const timeFormatOptions: any = { hour: 'numeric', minute: 'numeric' };

        // Format date and time separately using the adjusted local time
        const formattedDate = dateObject.toLocaleDateString(undefined, dateFormatOptions);
        const formattedTime = dateObject.toLocaleTimeString(undefined, timeFormatOptions);

        return `${formattedDate} ${formattedTime}`;
    }
}
