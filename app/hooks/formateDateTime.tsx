// Hook to take in one string date + time, and format it to Hour, Minute, Month, Day, and Year.
// Example: July 10th, 2023 1:00PM

export default function formatDateTime(dateTimeString: string): string | undefined {
    if (dateTimeString) {
        // Split the dateTimeString into dateString and timeString
        const [datePart, timePart] = dateTimeString.split(' ');
        const dateObject = new Date(datePart + 'T' + timePart);

        // Define your local timezone offset in minutes (UTC -1 in this case)
        const localTimezoneOffset = -300;

        // Calculate the UTC time in your local timezone
        const localTime = new Date(dateObject.getTime() + localTimezoneOffset * 60 * 1000);

        // Format options for date and time
        const dateFormatOptions: any = { month: 'long', day: 'numeric', year: 'numeric' };
        const timeFormatOptions: any = { hour: 'numeric', minute: 'numeric' };

        // Format date and time separately using the local time
        const formattedDate = localTime.toLocaleDateString(undefined, dateFormatOptions);
        const formattedTime = localTime.toLocaleTimeString(undefined, timeFormatOptions);

        return `${formattedDate} ${formattedTime}`;
    }
}