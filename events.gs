// The list of calendars I want to fetch information for
const calendarIds = [
  '<your personal calendar id>',
];

// Arguments used to fetch calendar information. Change as needed. See
// https://developers.google.com/calendar/v3/reference/events/list for more info
const optionalParams = {
  showDeleted: false,
  singleEvents: true,
  orderBy: 'startTime',
};

const doGet = (event = {}) => {
  const params = {...optionalParams, ...event};
  const date = new Date();
  if (!params.hasOwnProperty('timeMin')) {
    params['timeMin'] = (new Date(date.getFullYear(), date.getMonth(), 1)).toISOString();
  };
  if (!params.hasOwnProperty('timeMax')) {
    params['timeMax'] = (new Date(date.getFullYear(), date.getMonth() + 1, 0)).toISOString();
  };
  const events = calendarIds.flatMap(item => Calendar.Events.list(item, params).items);
  const simpleEvents = events.map(event => ({
    'summary': event.summary,
    'description': event.description || '',
    'start': event.start.dateTime ? event.start.dateTime : event.start.date,
    'end': event.end.dateTime ? event.end.dateTime : event.end.date,
    'call': event.hangoutLink
  }));
  return ContentService.createTextOutput(JSON.stringify(simpleEvents)).setMimeType(ContentService.MimeType.JSON);
};
