import { test, expect } from '@playwright/test';
import { exec } from 'node:child_process';
import { spawn } from 'node:child_process';
import { promisify } from 'node:util';

const execAsync = promisify(exec);

let serverProcess = null;

test.beforeAll(async () => {
  try {
    await execAsync('pkill -f "trunk serve" || true', { shell: true });
  } catch (e) {}
  
  try {
    serverProcess = spawn('trunk', ['serve'], {
      stdio: 'ignore',
      detached: true
    });
    serverProcess.unref();
    
    await new Promise(resolve => setTimeout(resolve, 5000));
  } catch (e) {
    console.error('Failed to start trunk server:', e);
  }
});

test.afterAll(async () => {
  try {
    if (serverProcess) {
      serverProcess.kill();
    }
    await execAsync('pkill -f "trunk serve" || true', { shell: true });
  } catch (e) {}
});

test('should render Telegram Mini App with styles', async ({ page }) => {
  await page.goto('http://localhost:8080');
  
  const appDiv = await page.$('#app');
  expect(appDiv).not.toBeNull();
  
  const appRoot = await appDiv.$('.tgui-app-root');
  expect(appRoot).not.toBeNull();
  
  const list = await appRoot.$('.tgui-list');
  expect(list).not.toBeNull();
  
  const section = await list.$('.tgui-section');
  expect(section).not.toBeNull();
  
  const cell = await section.$('.tgui-cell');
  expect(cell).not.toBeNull();
  
  const sectionHeader = await section.$('.tgui-section-header');
  expect(sectionHeader).not.toBeNull();
  const headerText = await sectionHeader.textContent();
  expect(headerText).toBeTruthy();
});
