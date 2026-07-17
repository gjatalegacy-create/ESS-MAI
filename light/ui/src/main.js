// NURA Legacy old UI v1.6.4 — vetëm upload + emocion sistemi.
const { invoke } = window.__TAURI__.core;

const form = document.getElementById("projectForm");
const filesInput = document.getElementById("projectFiles");
const fileSummary = document.getElementById("fileSummary");
const traceZone = document.getElementById("traceZone");
const uploadBtn = document.getElementById("uploadBtn");
const emotionOrb = document.getElementById("emotionOrb");
const emotionPhase = document.getElementById("emotionPhase");
const emotionMeta = document.getElementById("emotionMeta");

filesInput.addEventListener("change", () => {
    const names = Array.from(filesInput.files || []).map((file) => file.name);
    fileSummary.textContent = names.length === 0
        ? "Asnjë skedar i zgjedhur"
        : `${names.length} skedarë: ${names.join(", ")}`;
});

form.addEventListener("submit", async (event) => {
    event.preventDefault();
    uploadBtn.disabled = true;
    showTrace("→ UI ia dorëzon materialin vetëm Light-it...");

    try {
        const files = [];
        for (const file of Array.from(filesInput.files || [])) {
            const bytes = Array.from(new Uint8Array(await file.arrayBuffer()));
            files.push({
                name: file.name,
                mimeType: file.type || "application/octet-stream",
                bytes,
            });
        }

        const request = {
            projectName: value("projectName"),
            projectDescription: value("projectDescription"),
            projectContent: value("projectContent"),
            domain: value("domain"),
            hypothesis: value("hypothesis"),
            assumptions: value("assumptions").split(/\r?\n/).filter(Boolean),
            documentationDescription: value("documentationDescription"),
            files,
        };

        const result = await invoke("upload_project", { request });
        const state = result.accepted_into_gcl
            ? "PROJECT_ACCEPTED_UNDER_GCL"
            : `PROJECT_REJECTED_REASON_${result.reason_code}`;
        const emotional = await invoke("reflect_system_emotion", {
            traceId: result.trace_id,
            runtimeOutput: state,
        });
        applyEmotion(emotional);

        if (result.accepted_into_gcl) {
            showTrace(`✓ project=${result.project_id} trace=${result.trace_id}`);
            showTrace(`✓ Light route u pranua; context=${result.context_sha256}`);
            showTrace(`✓ skedarë të dorëzuar=${result.uploaded_files}`);
        } else {
            showTrace(`✗ Light e refuzoi intake-in reason=${result.reason_code}`);
        }
    } catch (error) {
        showTrace(`✗ ${error}`);
        try {
            const emotional = await invoke("reflect_system_emotion", {
                traceId: 0,
                runtimeOutput: "PROJECT_UPLOAD_ERROR",
            });
            applyEmotion(emotional);
        } catch (_) {
            // UI nuk shpik fallback emocional kur backend-i mungon.
        }
    } finally {
        uploadBtn.disabled = false;
    }
});

function value(id) {
    return document.getElementById(id).value.trim();
}

function showTrace(message) {
    const entry = document.createElement("div");
    entry.className = "trace-entry";
    entry.textContent = message;
    traceZone.appendChild(entry);
    traceZone.scrollTop = traceZone.scrollHeight;
}

function applyEmotion(command) {
    emotionPhase.textContent = command.phase;
    emotionMeta.textContent = `trust=${Math.round(command.trust_mass / 100)}% · ${command.motion} · ${command.animation}`;
    emotionOrb.dataset.motion = command.motion;
    emotionOrb.style.setProperty("--emotion-color", command.color);
}
