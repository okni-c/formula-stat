// Hook to take in a string date and time, and format it to Month, Day, and Year.
// Example: July 10th, 2023

export default function formatDate(dateString: string, timeString: string): string {
    const dateObject = new Date(dateString + 'T' + timeString);
    const options: any = { month: 'long', day: 'numeric', year: 'numeric' };
    return dateObject.toLocaleDateString(undefined, options);
}