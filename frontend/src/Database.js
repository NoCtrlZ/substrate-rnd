import React, { useEffect, useState } from 'react';
import { Grid, Item } from 'semantic-ui-react';

export default function Extrinsics (props) {

  return (
    <Grid.Column>
      <h1>Data</h1>
      <Item.Group>
        <Item>
          <Item.Content>
            <Item.Header>People</Item.Header>
          </Item.Content>
        </Item>
      </Item.Group>
    </Grid.Column>
  );
}
