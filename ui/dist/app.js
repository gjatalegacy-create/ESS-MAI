"use strict";

const tauriInvoke = window.__TAURI__?.core?.invoke ?? null;
const state = {
  deepResearch: false,
  selectedDomain: "Computer Science",
  files: [],
  activeView: "home"
};

const byId = (id) => document.getElementById(id);
const conversation = byId("conversation");
const question = byId("question");
const send = byId("send");
const deepResearch = byId("deepResearch");
const runtimePill = byId("runtimePill");
const projectResult = byId("projectResult");
const knowledgeFiles = byId("knowledgeFiles");
const fileList = byId("fileList");

function message(type, text) {
  const node = document.createElement("div");
  node.className = `message ${type}`;
  node.textContent = text;
  conversation.appendChild(node);
  conversation.scrollTop = conversation.scrollHeight;
}

function setView(name) {
  state.activeView = name;
  document.querySelectorAll(".view").forEach((view) => view.classList.remove("active-view"));
  document.querySelectorAll("[data-view]").forEach((button) => {
    button.classList.toggle("active", button.dataset.view === name);
    if (button.classList.contains("living-project")) {
      button.classList.toggle("active-project", name === "project");
    }
  });
  byId(name).classList.add("active-view");
  const titles = {
    home: "Home",
    domains: "Research Domains",
    knowledge: "My Light Knowledge",
    evolution: "My Evolution",
    architecture: "System Modules",
    project: "My Living Project"
  };
  byId("viewTitle").textContent = titles[name] ?? "Nura Legacy";
}

function applyEmotionalCommand(command) {
  if (!command) return;
  byId("emotionPhase").textContent = command.phase || "LIGHT_COORDINATION";
  byId("emotionSource").textContent = command.source || "OLD_UI_EMOTIONAL_ENGINE";
  byId("lastPhase").textContent = command.phase || "LIGHT_COORDINATION";
  const mass = Math.min(100, Math.round(Number(command.intensity_mass || 0) / 100));
  byId("emotionMeter").style.width = `${mass}%`;
  const palette = {
    NURA_GOLD: "#e5bd6a",
    LIGHT_CYAN: "#64d7ff",
    SHADOW_VIOLET: "#9f7cff",
    SPINE_BLUE: "#5d8dff",
    QUANTUM_BLUE: "#2f7fff",
    LIGHT_AMBER: "#d7a94c"
  };
  const color = palette[command.color] || palette.LIGHT_AMBER;
  byId("orb").style.background = color;
  byId("orb").style.boxShadow = `0 0 30px ${color}99`;
  byId("emotionMeter").style.background = color;
  byId("orbRing").style.borderColor = `${color}99`;
}

async function refreshStatus() {
  if (!tauriInvoke) {
    runtimePill.className = "runtime-pill bad";
    runtimePill.querySelector("span").textContent = "Open through the Tauri 2 binary";
    return;
  }
  try {
    const status = await tauriInvoke("runtime_status");
    runtimePill.className = `runtime-pill ${status.light_available ? "ok" : "bad"}`;
    runtimePill.querySelector("span").textContent = status.light_available
      ? "Light runtime connected — UI boundary enforced"
      : status.light_path;
  } catch (error) {
    runtimePill.className = "runtime-pill bad";
    runtimePill.querySelector("span").textContent = String(error);
  }
}

async function askNura() {
  const text = question.value.trim();
  if (!text) return;
  question.value = "";
  send.disabled = true;
  message("user", `You › ${text}`);
  message("system", "Tauri 2 is handing the request to Light. No direct Quantum/Shadow call.");
  if (!tauriInvoke) {
    message("error", "Tauri bridge unavailable. Open the compiled essmai_ui binary.");
    send.disabled = false;
    return;
  }
  try {
    const response = await tauriInvoke("ask_nura", {
      text,
      deepResearch: state.deepResearch
    });
    byId("lastTrace").textContent = String(response.trace_id ?? "—");
    applyEmotionalCommand(response.emotional);
    if (response.nura_verified) {
      message("nura", `Nura › ${response.nura_text}`);
    } else {
      message("system", "The cycle was reflected, but PD Light did not release verified Nura text.");
    }
  } catch (error) {
    message("error", `Nura › ${String(error)}`);
  } finally {
    send.disabled = false;
    question.focus();
  }
}

function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  return `${(bytes / 1024 / 1024).toFixed(2)} MiB`;
}

function renderFiles() {
  fileList.textContent = "";
  state.files.forEach((file) => {
    const row = document.createElement("div");
    row.className = "file-chip";
    const name = document.createElement("span");
    name.textContent = file.name;
    const size = document.createElement("span");
    size.textContent = formatBytes(file.size);
    row.append(name, size);
    fileList.appendChild(row);
  });
}

function setFiles(fileCollection) {
  const files = Array.from(fileCollection || []);
  const total = files.reduce((sum, file) => sum + file.size, 0);
  if (files.length > 8) {
    message("error", "Project intake allows at most 8 files.");
    return;
  }
  if (files.some((file) => file.size > 8 * 1024 * 1024)) {
    message("error", "Each project file must be 8 MiB or smaller.");
    return;
  }
  if (total > 24 * 1024 * 1024) {
    message("error", "Project files together must be 24 MiB or smaller.");
    return;
  }
  state.files = files;
  renderFiles();
}

async function serializeFiles() {
  const output = [];
  for (const file of state.files) {
    const buffer = await file.arrayBuffer();
    output.push({
      name: file.name,
      mime_type: file.type || "application/octet-stream",
      bytes: Array.from(new Uint8Array(buffer))
    });
  }
  return output;
}

async function submitProject(event) {
  event.preventDefault();
  if (!tauriInvoke) {
    projectResult.textContent = "Tauri bridge unavailable.";
    return;
  }
  const button = byId("submitProject");
  button.disabled = true;
  projectResult.textContent = "Preparing bounded project handoff to Light…";
  try {
    const route = document.querySelector('input[name="route"]:checked').value;
    const assumptions = byId("projectAssumptions").value
      .split(/\r?\n/)
      .map((value) => value.trim())
      .filter(Boolean);
    const request = {
      route,
      project_name: byId("projectName").value,
      project_description: byId("projectDescription").value,
      project_content: byId("projectContent").value,
      domain: byId("projectDomain").value,
      language_code: "sq",
      evolution_summary: byId("projectEvolution").value,
      hypothesis: byId("projectHypothesis").value,
      assumptions,
      documentation_description: byId("projectDocumentation").value,
      files: await serializeFiles()
    };
    const response = await tauriInvoke("submit_project", { request });
    byId("lastProjectRoute").textContent = response.route;
    projectResult.textContent = [
      `accepted=${response.accepted}`,
      `route=${response.route}`,
      `reason_code=${response.reason_code}`,
      `project_id=${response.project_id ?? "—"}`,
      `trace_id=${response.trace_id ?? "—"}`,
      `revision=${response.revision ?? "—"}`,
      `context_sha256=${response.context_sha256 ?? "—"}`,
      `content_sha256=${response.content_sha256 ?? "—"}`,
      `message=${response.message}`,
      "authority=LIGHT_COORDINATION_ONLY",
      "token_policy=UNCHANGED"
    ].join("\n");
    if (response.accepted) message("system", `Project accepted through Light route: ${response.route}.`);
  } catch (error) {
    projectResult.textContent = `Project submission failed:\n${String(error)}`;
  } finally {
    button.disabled = false;
  }
}

document.querySelectorAll("[data-view]").forEach((button) => {
  button.addEventListener("click", () => setView(button.dataset.view));
});

document.querySelectorAll("#domainGrid button").forEach((button) => {
  button.addEventListener("click", () => {
    state.selectedDomain = button.dataset.domain;
    byId("selectedDomain").textContent = state.selectedDomain;
    byId("projectDomain").value = state.selectedDomain;
    document.querySelectorAll("#domainGrid button").forEach((item) => item.classList.remove("selected"));
    button.classList.add("selected");
  });
});

send.addEventListener("click", askNura);
question.addEventListener("keydown", (event) => {
  if (event.key === "Enter") askNura();
});
deepResearch.addEventListener("click", () => {
  state.deepResearch = !state.deepResearch;
  deepResearch.classList.toggle("active", state.deepResearch);
  deepResearch.setAttribute("aria-pressed", String(state.deepResearch));
});
byId("attachButton").addEventListener("click", () => {
  setView("knowledge");
  knowledgeFiles.click();
});
knowledgeFiles.addEventListener("change", () => setFiles(knowledgeFiles.files));
byId("projectForm").addEventListener("submit", submitProject);

message("system", "Nura Legacy is ready. The active boundary is UI → Tauri 2 → Light.");
setView("home");
refreshStatus();
question.focus();
