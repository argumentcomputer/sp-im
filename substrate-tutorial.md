# How to Use `sp-im` within Substrate

1. Clone the [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
2. Navigate to `pallets/template/Cargo.toml` and import `sp-im` and `sp-std` as follows:
   ```rust
   [dependencies.sp-im]
   default-features = false
   version = '0.1'
   ```
   Add `sp-im`'s `std` feature
   ```rust
   'frame-benchmarking/std'
   'sp-im/std', # ADD
   ```
3. Navigate to `pallets/template/src/lib.rs` and make the following changes: 
   ```rust
   use frame_system::pallet_prelude::*;
   use sp_im::Vector; // ADD
   use sp_std::vec::Vec; // ADD
   ```
   Replace the `#[pallet::storage]` section with:
   ```rust
   #[pallet::storage]
   #[pallet::getter(fn vec)]
   // Learn more about declaring storage items:
   // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
   pub(super) type StorageVector<T> = StorageValue<_, Vec<u8>>;
   ```
   Replace the `#[pallet::event]` section with:
   ```rust
   #[pallet::event]
   #[pallet::metadata(T::AccountId = "AccountId")]
   pub enum Event<T: Config> {
     VectorStored(T::AccountId, Vec<u8>),
     VectorRetrieved(T::AccountId, Vec<u8>),
   }
   ```
   Replace the `#[pallet::call]` section with:
   ```rust
   #[pallet::call]
   impl<T: Config> Pallet<T> {
     /// An example dispatchable that takes a singles value as a parameter,
     /// writes the value to storage and emits an event. This function must
     /// be dispatched by a signed extrinsic.
     #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
     pub fn push_front_vector(
       origin: OriginFor<T>,
       input: u8,
     ) -> DispatchResult {
       let who = ensure_signed(origin)?;

       let stored_vec: Vec<u8> = match <StorageVector<T>>::get() {
         Some(vec) => vec,
         None => Vec::new(),
       };

       // Convert Vec into Vector for push_front
       let mut im_vec = Vector::from(stored_vec);
       im_vec.push_front(input);

       // Convert back to Vec for serialization
       let stored_vec: Vec<u8> = im_vec.into_iter().collect::<Vec<_>>();

       runtime_print!("Pushed the value {} onto vector {:?}", input, stored_vec);

       <StorageVector<T>>::put(stored_vec);

       runtime_print!("Request sent by: {:?}", who);

       Ok(())
     }

     /// An example dispatchable that may throw a custom error.
     #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
     pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
       let _who = ensure_signed(origin)?;

       // Read a value from storage.
       match <StorageVector<T>>::get() {
         // Return an error if the value has not been set.
         None => Err(Error::<T>::NoneValue)?,
         Some(_) => Ok(()),
       }
     }
   }
   ```

4. Build and run the node
   ```
   cargo build --release
   # Run a temporary node in development mode
   ./target/release/node-template --dev --tmp -lruntime=debug 
   ```
   Stop the node for now after making sure it works.

5. Clone the [Substrate Front End Template](https://github.com/substrate-developer-hub/substrate-front-end-template.git) in a separate directory.

6. Navigate to `src/TemplateModule.js` and replace it with the following:
   ```javascript
   import React, { useEffect, useState } from 'react';
   import { Form, Input, Grid } from 'semantic-ui-react';
   
   import { useSubstrate } from './substrate-lib';
   import { TxButton } from './substrate-lib/components';
   
   export function Main (props) {
     const { api } = useSubstrate();
     const { accountPair } = props;
 
     // The transaction submission status
     const [status, setStatus] = useState('');
 
     const [formValue, setFormValue] = useState(0);
 
     return (
       <Grid.Column width={20} style={{ textAlign: 'center' }}>
         <h1>Vector Storage</h1>
         <Form>
           <Form.Field>
             <Input
               label='Input a number'
               type='number'
               onChange={(_, { value }) => setFormValue(value)}
             />
           </Form.Field>
           <Form.Field style={{ textAlign: 'center' }}>
             <TxButton
               accountPair={accountPair}
               label='Vector Push Front'
               type='SIGNED-TX'
               setStatus={setStatus}
               attrs={{
                 palletRpc: 'templateModule',
                 callable: 'pushFrontVector',
                 inputParams: [formValue],
                 paramFields: [true]
               }}
             />
           </Form.Field>
           <div style={{ overflowWrap: 'break-word' }}>{status}</div>
         </Form>
       </Grid.Column>
     );
   }
 
   export default function TemplateModule (props) {
     const { api } = useSubstrate();
     return api.query.templateModule && api.query.templateModule.storageVector
       ? <Main {...props} />
       : null;
   }
   ```

7. Build and install the front end as detailed in the [Readme](https://github.com/substrate-developer-hub/substrate-front-end-template/blob/master/README.md)
8. Run the node as done in Step 4, then run the front end with `yarn start`
9. Interact with the `sp-im` commands
  * In the browser, scroll to the bottom of the page and input an integer, then hit "Vector Push Front". The resulting vector should appear in the node's `stdout`.
   
## Working example of this tutorial
See the Yatima [substrate-node-template](https://github.com/yatima-inc/substrate-node-template) and
[substrate-front-end-template](https://github.com/yatima-inc/substrate-front-end-template) repos on the `im-tutorial` branch
