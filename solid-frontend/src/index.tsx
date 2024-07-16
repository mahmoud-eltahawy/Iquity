/* @refresh reload */
import { render } from 'solid-js/web'
import { Router, Route } from "@solidjs/router";

import './index.css'
import {Editor,Preview} from './App'

const root = document.getElementById('root')

render(() => (
  <Router>
    <Route path="/editor" component={Editor}/>
    <Route path="/preview" component={Preview}/>
  </Router>
), root!)
