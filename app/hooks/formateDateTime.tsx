// Hook to take in one string date + time, and format it to Hour, Minute, Month, Day, and Year.
// Example: July 10th, 2023 1:00PM

export default function formatDateTime(dateTimeString: string): string | undefined {
    if (dateTimeString) {
        // Split the dateTimeString into dateString and timeString
        const [datePart, timePart] = dateTimeString.split(' ');
        const dateObject = new Date(datePart + 'T' + timePart);
        
        // Format options for date and time
        const dateFormatOptions: any = { month: 'long', day: 'numeric', year: 'numeric' };
        const timeFormatOptions: any = { hour: 'numeric', minute: 'numeric' };
        
        // Format date and time separately
        const formattedDate = dateObject.toLocaleDateString(undefined, dateFormatOptions);
        const formattedTime = dateObject.toLocaleTimeString(undefined, timeFormatOptions);

        return `${formattedDate} ${formattedTime}`;
    }
}


