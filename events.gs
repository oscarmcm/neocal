// The list of calendars I want to fetch information for
const calendarIds = [
  '<your calendar ids>',
];

// Arguments used to fetch calendar information. Change as needed. See
// https://developers.google.com/calendar/v3/reference/events/list for more info
const optionalParams = {
  showDeleted: false,
  singleEvents: true,
  orderBy: 'startTime',
};

const doGet = (event = {}) => {
  const params = {...optionalParams, ...event.parameter};
  const date = new Date();

  if (!params.hasOwnProperty('timeMin')) {
    params['timeMin'] = (new Date(date.getFullYear(), date.getMonth(), 1)).toISOString();
  };
  if (!params.hasOwnProperty('timeMax')) {
    params['timeMax'] = (new Date(date.getFullYear(), date.getMonth() + 1, 0)).toISOString();
  };

  const calendarEvents = calendarIds.flatMap(item => Calendar.Events.list(item, params).items);
  const formattedEvents = calendarEvents.map(event => {
    let start_date = new Date(event.start.dateTime ? event.start.dateTime : event.start.date);
    let end_date = new Date(event.start.dateTime ? event.start.dateTime : event.start.date);
    return {
      'summary': event.summary,
      'description': event.description || '',
      'start_date': start_date.toDateString(),
      'start_date_time': event.start.dateTime || '',
      'end_date': end_date.toDateString(),
      'end_date_time': event.end.dateTime || '',
      'call': event.hangoutLink || '',
    };
  });

  return ContentService.createTextOutput(
    JSON.stringify(formattedEvents)
  ).setMimeType(ContentService.MimeType.JSON);
};

