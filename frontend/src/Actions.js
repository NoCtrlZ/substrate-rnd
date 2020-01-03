import React, { useEffect, useState } from 'react';
import { Grid, Form, Dropdown, Input } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

export default function Actions (props) {
  const { api } = useSubstrate();
  const [modulesList, setModulesList] = useState([]);
  console.log(modulesList);
  const [status, setStatus] = useState(null);
  const [callableFunctionList, setCallableFunctionList] = useState([]);
  const { accountPair } = props;

  const [formState, setFormState] = useState({
    module: 'government',
    callableFunction: '',
    input: ''
  });
  console.log("This is setForm")
  console.log(formState.module)
  const { module, callableFunction, input } = formState;

  useEffect(() => {
    const modules = Object.keys(api.tx)
      .sort()
      .map(module => ({
        key: module,
        value: module,
        text: module
      }));

    setModulesList(modules);
  }, [api]);

  useEffect(() => {
    if (module !== '') {
      const callableFunctions = Object.keys(api.tx[module])
        .sort()
        .map(callable => ({
          key: callable,
          value: callable,
          text: callable
        }));

      setCallableFunctionList(callableFunctions);
    }
  }, [api, module]);

  const onChange = (_, data) =>
    setFormState(formState => ({ ...formState, [data.state]: data.value }));

  return (
    <Grid.Column>
      <h1>Actions</h1>
      <Form>
        <Form.Field>
          <Dropdown
            placeholder='Select a module to call'
            fluid
            label='Module'
            onChange={onChange}
            search
            selection
            state='module'
            options={modulesList}
          />
        </Form.Field>
        <Form.Field>
          <Dropdown
            placeholder='Select a function to call'
            fluid
            label='Callable Function'
            onChange={onChange}
            search
            selection
            state='callableFunction'
            options={callableFunctionList}
          />
        </Form.Field>
        <Form.Field>
          <Input
            onChange={onChange}
            label='Input'
            fluid
            placeholder='May not be needed'
            state='input'
            type='text'
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Call'
            setStatus={setStatus}
            type='TRANSACTION'
            attrs={{
              params: input ? [input] : null,
              tx: api.tx[module] && api.tx[module][callableFunction]
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  );
}
