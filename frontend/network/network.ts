export const requestOptionsGET = {
  method: 'GET',
  headers: {
    'Content-Type': 'application/json',
  },
};

export function requestOptionsPOST(body: any) {
  const requestOptions = {
    method: 'Post',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(body)
  };

  return requestOptions;
}

export function requestOptionsDELETE(body: any) {
  const requestOptions = {
    method: 'Delete',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(body)
  };

  return requestOptions;
}