package io.marcopolo.pietdemo

import android.graphics.DashPathEffect
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.system.Os
import android.view.Gravity
import android.view.ViewGroup
import android.widget.Button

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("pietdemo")
        initRust(cacheDir.absolutePath)
        setContentView(R.layout.activity_main)
        val pietView = findViewById<PietDemoView>(R.id.pietView)
        val nextButton = findViewById<Button>(R.id.nextTest)
//        val f = floatArrayOf(1.0F)
//        val testDash = DashPathEffect(f, 0.0F)

//        val pietView = PietDemoView(this);
//        val pietView = this.resources.getId
//        val nextButton = Button(this);

        nextButton.text = "Next test picture"
        nextButton.setOnClickListener {
            drawNext()
            pietView.invalidate()
        }

//        addContentView(PietDemoView(this))
//        addContentView(nextButton, ViewGroup.LayoutParams(
//            ViewGroup.LayoutParams.MATCH_PARENT,
//            ViewGroup.LayoutParams.WRAP_CONTENT
//        ));


    }

    external fun initRust(cachePath: String)

    external fun drawNext()
}
